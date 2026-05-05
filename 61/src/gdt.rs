#![allow(static_mut_refs)]

static mut GDT: x86_64::structures::gdt::GlobalDescriptorTable =
    x86_64::structures::gdt::GlobalDescriptorTable::new();

const STACK_SIZE: usize = 4096 * 5;

static mut TSS: x86_64::structures::tss::TaskStateSegment =
    x86_64::structures::tss::TaskStateSegment::new();

static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

pub fn init_gdt() {
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] =
            x86_64::VirtAddr::from_ptr(&raw const STACK) + STACK_SIZE;

        let kcs = GDT.add_entry(x86_64::structures::gdt::Descriptor::kernel_code_segment());
        let tss = GDT.add_entry(x86_64::structures::gdt::Descriptor::tss_segment(&TSS));

        GDT.load();

        use x86_64::instructions::segmentation::Segment;
        x86_64::instructions::segmentation::CS::set_reg(kcs);
        x86_64::instructions::tables::load_tss(tss);
    }
}
