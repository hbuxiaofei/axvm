searchState.loadedDescShard("axvm", 0, "This crate provides a minimal VM monitor (VMM) for running …\nA reference to a vCPU.\nA Virtual Machine.\nThe interfaces which the underlying software (kernel or …\nThe architecture-independent per-CPU type.\nA reference to a VM.\nThe low-level <strong>OS-dependent</strong> helpers that must be provided …\nBoots the VM by setting the running flag as true.\nThe configuration structure for the VM. The <code>AxVMCrateConfig</code>…\nCurrent time in nanoseconds.\nReturns the base address of the two-stage address …\nReturns the argument unchanged.\nReturns this VM’s emulated devices.\nReturns guest VM image load region in …\nWhether the hardware has virtualization support.\nReturns the VM id.\nCalls <code>U::from(self)</code>.\nCreates a new VM with the given configuration. Returns an …\nRun a vCPU according to the given vcpu_id.\nReturns if the VM is running.\nRetrieves the vCPU corresponding to the given vcpu_id for …\nReturns a reference to the list of vCPUs corresponding to …\nReturns the number of vCPUs corresponding to the VM.\nConverts a virtual address to the corresponding physical …\nA part of <code>AxVCpuConfig</code>, which represents an …\nA part of <code>AxVMCrateConfig</code>, which represents a <code>VM</code>.\nThe configuration structure for the guest VM serialized …\nA part of <code>AxVMConfig</code>, which stores configuration …\nHost VM, used for boot from Linux like Jailhouse do, named …\nGuest Linux, generally a full-featured guest OS with …\nGuest RTOS, generally a simple guest OS with most of the …\nA part of <code>AxVMConfig</code>, which represents guest VM type.\nA part of <code>AxVMConfig</code>, which represents a memory region.\nReturns the entry address in GPA for the Application …\nThe entry address in GPA for the Application Processor …\nThe load address of the BIOS image, <code>None</code> if not used.\nThe load address in GPA for the BIOS image, <code>None</code> if not …\nThe file path of the BIOS image, <code>None</code> if not used.\nReturns the entry address in GPA for the Bootstrap …\nThe entry address in GPA for the Bootstrap Processor (BSP).\nThe load address of the device tree blob (DTB), <code>None</code> if …\nThe load address in GPA for the device tree blob (DTB), …\nThe file path of the device tree blob (DTB), <code>None</code> if not …\nReturns configurations related to VM emulated devices.\nThe mappings flags of the memory region, refers to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserialize the toml string to <code>AxVMCrateConfig</code>.\nReturns vCpu id list and its corresponding pCpu affinity …\nThe start address of the memory region in GPA.\nReturns VM id.\nReturns configurations related to VM image load addresses.\nThe location of the image, default is ‘fs’.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe load address of the kernel image.\nThe load address in GPA for the kernel image.\nThe file path of the kernel image.\nReturns configurations related to VM memory regions.\nReturns VM name.\nThe load address of the ramdisk image, <code>None</code> if not used.\nThe load address in GPA for the ramdisk image, <code>None</code> if not …\nThe file path of the ramdisk image, <code>None</code> if not used.\nThe size of the memory region.")