#![allow(unsafe_code, unused)]
#![no_main]
#![no_std]

use core::{borrow::BorrowMut, fmt::Result};
use core::ptr;

use cortex_m::asm::{nop, self};
use panic_halt as _;

use cortex_m_rt::entry;
use::core::arch::asm;
use cortex_m_semihosting::{debug, hprintln};
use stm32f3xx_hal_v2::{flash::ACR, pac::Peripherals, pac::FLASH};

const UNLOCK_KEY1: u32 = 0x4567_0123;
const UNLOCK_KEY2: u32 = 0xCDEF_89AB;

fn unlock(flash: &mut FLASH) ->bool{

    if flash.cr.read().lock().bit_is_clear(){
        return true;
    }

    flash.keyr.write(|w| unsafe { w.bits(UNLOCK_KEY1) });
    flash.keyr.write(|w| unsafe { w.bits(UNLOCK_KEY2) });

    if flash.cr.read().lock().bit_is_clear() {
        return true;
    } else {
        return false;
    }
}

fn wait_ready(flash: &FLASH) {
    while flash.sr.read().bsy().bit() {}
}

fn clear_error_flags(flash: &FLASH) {
    if flash.sr.read().wrprterr().bit_is_set() {
        flash.sr.modify(|_, w| w.wrprterr().set_bit());
    }
    if flash.sr.read().pgerr().bit_is_set() {
        flash.sr.modify(|_, w| w.pgerr().set_bit());
    }
}

fn erase_page(flash: &mut FLASH, page: u32){

    // 1. Check that no Flash memory operation is ongoing by checking the BSY bit in the Flash
    // status register (FLASH_SR).
    if flash.sr.read().bsy().bit_is_set() {
        hprintln!("Flash is busy.");
     }

    // 2. Check and clear all error programming flags due to a previous programming. If not,
     // PGSERR is set.
    clear_error_flags(&flash);

    // 3. Set the PER bit and select the page you wish to erase (PNB). For dual bank variants:
     //  - with the associated bank(BKER) in the Flash control register (FLASH_CR).

     flash.cr.modify(|_, w| w.per().set_bit());

     // Program the FLASH_CR register
     // regs.ar.modify(|_, w| w.far().bits(page as u8));
     flash.ar.write(|w| unsafe { w.bits(page as u32) });


     // 4. Set the STRT bit in the FLASH_CR register.
     flash.cr.modify(|_, w| w.strt().set_bit());

    // 5. Wait for the BSY bit to be cleared in the FLASH_SR register.
      
    while flash.sr.read().bsy().bit_is_set() {}

    // 6. lock the flash
    while flash.sr.read().bsy().bit_is_set() {}
    flash.cr.modify(|_, w| w.lock().set_bit());

}

fn write_to_flash(flash: &mut FLASH, addr: u32, data: u32) {
        unlock(flash);

        // 1. Check that no Flash memory operation is ongoing by checking the BSY bit in the Flash
        if flash.sr.read().bsy().bit_is_set() {
            hprintln!("Flash is busy.");
        }
         
        clear_error_flags(&flash);
       // 2. Set the PG bit in the Flash control register (FLASH_CR).
       flash.cr.modify(|_, w| w.pg().set_bit());


        // 3. Perform the data write (half-word) at the desired address.
        unsafe{
                ptr::write_volatile(addr as *mut u16, data as u16);
                ptr::write_volatile((addr as usize + 2) as * mut u16, (data.wrapping_shr(16)) as u16);
        }

        // 4. Wait for the BSY bit to be cleared in the FLASH_SR register.

        while flash.sr.read().bsy().bit_is_set() {}
        // 5. lock the flash
        flash.cr.modify(|_, w| w.lock().set_bit());
        

        // 6. Check that EOP flag is set in the FLASH_SR register (meaning that the programming
        // operation has succeed), and clear it by software.
        if flash.sr.read().eop().bit_is_set() {
            flash.sr.modify(|_, w| w.eop().set_bit()); // Clear
        }

         // 6. Clear the PG bit in the FLASH_CR register if there no more programming request
        // anymore.
        flash.cr.modify(|_, w| w.pg().clear_bit());

}

fn checkpoint(){

    unsafe {
        asm!(
            "add sp, #80"
        );
    }
    unsafe {
        asm!(
            "pop {{r7}}"
        );
    }
    unsafe {
        asm!(
            "push {{r7}}"
        );
    }
    unsafe {
        asm!(
            "sub sp, #80"
        );
    }

    let r0_value: u32;
    let r1_value: u32;
    let r2_value: u32;
    let r3_value: u32;
    let r4_value: u32;
    let r5_value: u32;
    let r6_value: u32;
    let r7_value: u32;
    let r8_value: u32;
    let r9_value: u32;
    let r10_value: u32;
    let r11_value: u32;
    let r12_value: u32;
    let r13_sp: u32;
    let r14_lr: u32;
    let r15_pc: u32;

    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r0_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r1",
            out(reg) r1_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r2",
            out(reg) r2_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r3",
            out(reg) r3_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r4",
            out(reg) r4_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r5",
            out(reg) r5_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r6",
            out(reg) r6_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r7",
            out(reg) r7_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r8",
            out(reg) r8_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r9",
            out(reg) r9_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r10",
            out(reg) r10_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r11",
            out(reg) r11_value
        );
    }


    unsafe {
        asm!(
            "MOV {0}, r12",
            out(reg) r12_value
        );
    }
 
    unsafe {
        asm!(
            "MOV {0}, r14",
            out(reg) r14_lr
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r15",
            out(reg) r15_pc
        );
    }


    unsafe {
        asm!(
            "MOV r0, sp",
        );
    }
    unsafe {
        asm!(
            "add r0, #80",
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r13_sp
        );
    }
    let dp = Peripherals::take().unwrap();
    let mut flash= dp.FLASH;
    //let page = 0x0800_9034 as u32;
     unlock(& mut flash);
    // if status {
    //     hprintln!("Flash memory is unlocked.");
    // }
    wait_ready(&flash);
    //erase_page(&mut flash, 0x0800_9060 as u32);
    write_to_flash(&mut flash,  0x0800_9060 as u32, r0_value as u32);
    write_to_flash(&mut flash,  0x0800_9064 as u32, r1_value as u32);
    write_to_flash(&mut flash,  0x0800_9068 as u32, r2_value as u32);
    write_to_flash(&mut flash,  0x0800_906C as u32, r3_value as u32);
    write_to_flash(&mut flash,  0x0800_9070 as u32, r4_value as u32);
    write_to_flash(&mut flash,  0x0800_9074 as u32, r5_value as u32);
    write_to_flash(&mut flash,  0x0800_9078 as u32, r6_value as u32);
    write_to_flash(&mut flash,  0x0800_907C as u32, r7_value as u32);
    write_to_flash(&mut flash,  0x0800_9084 as u32, r8_value as u32);
    write_to_flash(&mut flash,  0x0800_9088 as u32, r9_value as u32);
    write_to_flash(&mut flash,  0x0800_908C as u32, r10_value as u32);
    write_to_flash(&mut flash,  0x0800_9090 as u32, r11_value as u32);
    write_to_flash(&mut flash,  0x0800_9094 as u32, r12_value as u32);
    write_to_flash(&mut flash,  0x0800_9098 as u32, r13_sp as u32);
    write_to_flash(&mut flash,  0x0800_909C as u32, r14_lr as u32);
    write_to_flash(&mut flash,  0x0800_90A0 as u32, r15_pc as u32);

     
        unsafe{
            // let r0_flash = ptr::read_volatile(0x0800_9060 as *const u32);
            // let r1_flash = ptr::read_volatile(0x0800_9064 as *const u32);
            // let r2_flash = ptr::read_volatile(0x0800_9068 as *const u32);
            // let r3_flash = ptr::read_volatile(0x0800_906C as *const u32);
            // let r4_flash = ptr::read_volatile(0x0800_9070 as *const u32);
            // let r5_flash = ptr::read_volatile(0x0800_9074 as *const u32);
            // let r6_flash = ptr::read_volatile(0x0800_9078 as *const u32);
            // let r7_flash = ptr::read_volatile(0x0800_907C as *const u32);

            
            // asm!("STMDB sp!, {{{0}, {1}, {2}}}", // Store Multiple with Decrement Before
            //         in(reg) r0_flash,
            //         in(reg) r1_flash,
            //         in(reg) r2_flash,
            //     );
        
            // asm!("MOV r1, {0}",in(reg) r1_flash);
            // asm!("MOV r2, {0}",in(reg) r2_flash);
            // asm!("MOV r3, {0}",in(reg) r3_flash);
            // asm!("MOV r4, {0}",in(reg) r4_flash);
            // asm!("MOV r5, {0}",in(reg) r5_flash);
        }
    
        
}

fn restore()->bool{
    unsafe {
        let r0_flash = ptr::read_volatile(0x0800_9060 as *const u32);
        if r0_flash == 0xffff_ffff {
            return false
        }
        asm!(
            "LDR r1, [{}]", 
            in(reg) 0x0800_9064 as u32
        );
        // asm!("Push {{r1}}");

        asm!(
            "LDR r2, [{}]", 
            in(reg) 0x0800_9068 as u32
        );
        // asm!("Push {{r2}}");

        asm!(
            "LDR r3, [{}]", 
            in(reg) 0x0800_906C as u32
        );
        // asm!("Push {{r3}}");

        asm!(
            "LDR r4, [{}]", 
            in(reg) 0x0800_9070 as u32
        );

        // asm!("Push {{r4}}");

        asm!(
            "LDR r5, [{}]", 
            in(reg) 0x0800_9074 as u32
        );  
        // asm!("Push {{r5}}");    
        
        asm!(
            "LDR r6, [{}]", 
            in(reg) 0x0800_9078 as u32
        );  
        // asm!("Push {{r6}}");

        asm!(
            "LDR r7, [{}]", 
            in(reg) 0x0800_907C as u32
        );

        asm!(
            "LDR r8, [{}]", 
            in(reg) 0x0800_9084 as u32
        );
        
        asm!(
            "LDR r9, [{}]", 
            in(reg) 0x0800_9088 as u32
        );  

        asm!(
            "LDR r10, [{}]", 
            in(reg) 0x0800_908C as u32
        );  

        asm!(
            "LDR r11, [{}]", 
            in(reg) 0x0800_9090 as u32
        );
        
        asm!(
            "LDR r12, [{}]", 
            in(reg) 0x0800_9094 as u32
        );

        asm!(
            "LDR sp, [{}]", 
            in(reg) 0x0800_9098 as u32
        );

        asm!(
            "LDR r0, [{}]", 
            in(reg) 0x0800_909C as u32
        );
        asm!("Push {{r0}}");
        
        asm!(
            "LDR r0, [{}]", 
            in(reg) 0x0800_9060 as u32
        );

        asm!("POP {{PC}}");   // restore LR to PC
    }
    return true;
}

fn delete_pg(page: u32){
    let dp = Peripherals::take().unwrap();
    let mut flash= dp.FLASH;
    //let page = 0x0800_9034 as u32;
    unlock(& mut flash); 
    wait_ready(&flash);
    erase_page(&mut flash,  page);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    
   restore();
  //delete_pg(0x0800_9060 as u32);
     unsafe {
        asm!("  mov r0, #10
                mov r1, #24
                mov r2, #16
                mov r3, #32
                mov r4, #67
                mov r5, #79
                mov r6, #59
                mov r7, #17
                ")
     }
    checkpoint();
    unsafe{
        asm!("add r0, r1");
    }
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);

   loop {}
}
