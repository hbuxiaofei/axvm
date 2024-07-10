// Copyright (c) 2023 Beihang University, Huawei Technologies Co.,Ltd. All rights reserved.
// Rust-Shyper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
// EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
// MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use alloc::vec::Vec;
use core::mem::size_of;
use core::arch::global_asm;
use spin::Mutex;
use core::marker::PhantomData;

// type ContextFrame = crate::arch::contextFrame::Aarch64ContextFrame;
use cortex_a::registers::*;
use tock_registers::interfaces::*;

use super::ContextFrame;
use super::VmContext;
use crate::AxvmHal;

core::arch::global_asm!(include_str!("entry.S"));
// use crate::arch::hvc::run_guest_by_trap2el2;

// TSC, bit [19]
const HCR_TSC_TRAP: usize = 1 << 19;

/// Vcpu State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcpuState {
    /// Invalid
    Inv = 0,
    /// Runnable
    Runnable = 1,
    /// Running
    Running = 2,
    /// Blocked
    Blocked = 3,
}


/// (v)CPU register state that must be saved or restored when entering/exiting a VM or switching
/// between VMs.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VmCpuRegisters {
    /// guest trap context
    pub trap_context_regs: ContextFrame,
    /// virtual machine system regs setting
    pub vm_system_regs: VmContext,
}

impl VmCpuRegisters {
    /// create a default VmCpuRegisters
    pub fn default() -> VmCpuRegisters {
        VmCpuRegisters {
            trap_context_regs: ContextFrame::default(),
            vm_system_regs: VmContext::default(),
        }
    }
}

/// A virtual CPU within a guest
#[derive(Clone, Debug)]
pub struct VCpu<H:AxvmHal> {
    /// Vcpu context
    pub regs: VmCpuRegisters,

    marker: PhantomData<H>,
}

extern "C" {
    fn context_vm_entry(ctx: usize) -> !;
}

type AxArchVCpuConfig = VmCpuRegisters;

// Public Function
impl <H:AxvmHal> VCpu<H> {
    /// Create a new vCPU
    pub fn new(_config: AxArchVCpuConfig) -> Self {
        // Self {
        //     regs: VmCpuRegisters::default(),
        //     marker: PhantomData,
        // }
        Self {
            regs: AxArchVCpuConfig::default(),
            marker: PhantomData,
        }
    }

    /// Set guest entry point
    pub fn set_entry(&mut self, entry: GuestPhysAddr) -> AxResult {
        self.set_elr(elr);
        Ok(())
    }

    /// Set ept root
    pub fn set_ept_root(&mut self, ept_root: HostPhysAddr) -> AxResult {
        self.regs.vm_system_regs.vttbr_el2 = ept_root as u64;
        Ok(())
    }

    /// Set device tree ipa
    pub fn set_dtb_ipa(&mut self, dtb_ipa: GuestPhysAddr) -> AxResult {
        self.set_gpr(0, device_tree_ipa);
        Ok(())
    }

    /// Run vcpu
    pub fn run(&mut self) -> AxResult<crate::vcpu::AxArchVCpuExitReason> {
        self.init_hv();
        unsafe {
            context_vm_entry(self.vcpu_trap_ctx_addr(true));
        }
        Ok(())
    }

    pub fn bind(&mut self) -> AxResult {
        unimplemented!()
    }

    pub fn unbind(&mut self) -> AxResult {
        unimplemented!()
    }

}

// Private function
impl <H:AxvmHal> VCpu<H> {
    fn init_hv(&mut self) {
        self.regs.trap_context_regs.spsr =( SPSR_EL1::M::EL1h + 
            SPSR_EL1::I::Masked + 
            SPSR_EL1::F::Masked + 
            SPSR_EL1::A::Masked + 
            SPSR_EL1::D::Masked )
            .value;
        self.init_vm_context();

        unsafe {
            core::arch::asm!("
                mov x3, xzr           // Trap nothing from EL1 to El2.
                msr cptr_el2, x3"
            );
        }
        self.regs.vm_system_regs.ext_regs_restore();
        unsafe {
            cache_invalidate(0<<1);
            cache_invalidate(1<<1);
            core::arch::asm!("
                ic  iallu
                tlbi	alle2
                tlbi	alle1         // Flush tlb
                dsb	nsh
                isb"
            );
        }
    }

    /// Init guest context. Also set some el2 register value.
    fn init_vm_context(&mut self) {
        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        self.regs.vm_system_regs.cntvoff_el2 = 0;
        self.regs.vm_system_regs.cntkctl_el1 = 0;

        self.regs.vm_system_regs.sctlr_el1 = 0x30C50830;
        self.regs.vm_system_regs.pmcr_el0 = 0;
        // self.regs.vm_system_regs.vtcr_el2 = 0x8001355c;
        self.regs.vm_system_regs.vtcr_el2 = (VTCR_EL2::PS::PA_40B_1TB   // 40bit PA, 1TB
                                          + VTCR_EL2::TG0::Granule4KB
                                          + VTCR_EL2::SH0::Inner
                                          + VTCR_EL2::ORGN0::NormalWBRAWA
                                          + VTCR_EL2::IRGN0::NormalWBRAWA
                                          + VTCR_EL2::SL0.val(0b01)
                                          + VTCR_EL2::T0SZ.val(64 - 40)).into();
        self.regs.vm_system_regs.hcr_el2 = (HCR_EL2::VM::Enable
                                         + HCR_EL2::RW::EL1IsAarch64 // ).into();
                                         + HCR_EL2::IMO::EnableVirtualIRQ).into();
        // trap el1 smc to el2
        self.regs.vm_system_regs.hcr_el2 |= HCR_TSC_TRAP as u64;

        let mut vmpidr = 0;
        vmpidr |= 1 << 31;
        vmpidr |= self.vcpu_id;
        self.regs.vm_system_regs.vmpidr_el2 = vmpidr as u64;
        // self.gic_ctx_reset(); // because of passthrough gic, do not need gic context anymore?
    }
    
    /// Get vcpu whole context address
    fn vcpu_ctx_addr(&self) -> usize {
        &(self.regs) as *const _ as usize
    }
    
    /// Get vcpu trap context for guest or arceos
    fn vcpu_trap_ctx_addr(&self) -> usize {
        &(self.regs.trap_context_regs) as *const _ as usize
    }

    /// Set exception return pc
    fn set_elr(&mut self, elr: usize) {
        self.regs.trap_context_regs.set_exception_pc(elr);
    }

    /// Get general purpose register
    fn get_gpr(&mut self, idx: usize) {
        self.regs.trap_context_regs.gpr(idx);
    }

    /// Set general purpose register
    fn set_gpr(&mut self, idx: usize, val: usize) {
        self.regs.trap_context_regs.set_gpr(idx, val);
    }
}

unsafe fn cache_invalidate(cache_level: usize) {
    core::arch::asm!(
        r#"
        msr csselr_el1, {0}
        mrs x4, ccsidr_el1 // read cache size id.
        and x0, x4, #0x7
        add x0, x0, #0x4 // x0 = cache line size.
        ldr x3, =0x7fff
        and x2, x3, x4, lsr #13 // x2 = cache set number – 1.
        ldr x3, =0x3ff
        and x3, x3, x4, lsr #3 // x3 = cache associativity number – 1.
        clz w4, w3 // x4 = way position in the cisw instruction.
        mov x5, #0 // x5 = way counter way_loop.
    // way_loop:
    1:
        mov x6, #0 // x6 = set counter set_loop.
    // set_loop:
    2:
        lsl x7, x5, x4
        orr x7, {0}, x7 // set way.
        lsl x8, x6, x0
        orr x7, x7, x8 // set set.
        dc csw, x7 // clean and invalidate cache line.
        add x6, x6, #1 // increment set counter.
        cmp x6, x2 // last set reached yet?
        ble 2b // if not, iterate set_loop,
        add x5, x5, #1 // else, next way.
        cmp x5, x3 // last way reached yet?
        ble 1b // if not, iterate way_loop
        "#,
        in(reg) cache_level,
        options(nostack)
    );
}
