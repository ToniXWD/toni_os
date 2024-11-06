use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// 中断栈表 （IST）实现的，IST是一个由7个确认可用的完好栈的指针组成的
// 中断栈表（IST）其实是一个名叫 任务状态段（TSS） 的古老遗留结构的一部分
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // 将IST的0号位定义为 double fault 的专属栈
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

// 创建一个包含了静态 TSS 段的 GDT 静态结构：
// 在全局描述符表（GDT）中添加一个段描述符，
// 然后就可以通过ltr 指令加上GDT序号加载我们的TSS
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // GDT是包含了程序 段信息 的结构
        // GDT在64位模式下已经不再受到支持，但其依然有两个作用，切换内核空间和用户空间，以及加载TSS结构。
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector); // 重载代码段寄存器 cs
        load_tss(GDT.1.tss_selector); // 重载 TSS 段
    }
}
