#![allow(unsafe_code, unused, non_upper_case_globals)]
#![no_main]
#![no_std]
use core::mem;
use core::ptr;
use cortex_m::asm::{nop, self};
use panic_halt as _;

use cortex_m_rt::entry;
use::core::arch::asm;
use cortex_m_semihosting::{debug, hprintln};
use stm32f3xx_hal_v2::{self as hal, pac, prelude::*,flash::ACR, pac::Peripherals, pac::FLASH};

use volatile::Volatile;
use core::sync::atomic::{compiler_fence, Ordering};
use stm32f3xx_hal_v2::hal::blocking::rng::Read;

mod my_flash;
use my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

static mut x:u8 = 1;
static mut y:u8 = 3;
static mut z:u8 = 2;
static mut t:u8 = 5; //change to assign a random number

const UNLOCK_KEY1: u32 = 0x4567_0123;
const UNLOCK_KEY2: u32 = 0xCDEF_89AB;
const FRAM_ADDRESS:u8 = 0x50;

const MEMORY_ADDRESS_1: u16 = 0x0000; //x Address to write/read data to/from
const MEMORY_ADDRESS_2: u16 = 0x0001; //y Address to write/read data to/from
const MEMORY_ADDRESS_3: u16 = 0x0002; //z Address to write/read data to/from
const MEMORY_ADDRESS_4: u16 = 0x0003; //t Address to write/read data to/from

static  mut back_up_address:u16 = 0x0100; //location to checkpoint data

fn init_i2c() -> hal::i2c::I2c<pac::I2C1, (stm32f3xx_hal_v2::gpio::gpiob::PB8<hal::gpio::AF4>, stm32f3xx_hal_v2::gpio::gpiob::PB9<hal::gpio::AF4>)> {
    unsafe{
    let dp =  Peripherals::steal();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
    let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let pins = (scl, sda);

    let mut i2c = hal::i2c::I2c::new(dp.I2C1, pins, 100.khz(), clocks, &mut rcc.apb1);
    i2c
    }
}

fn write_data(i2c: &mut hal::i2c::I2c<pac::I2C1, (stm32f3xx_hal_v2::gpio::gpiob::PB8<hal::gpio::AF4>, stm32f3xx_hal_v2::gpio::gpiob::PB9<hal::gpio::AF4>)>, address: u8, memory_address: u16, value: u8) {
    let buff = [
        ((memory_address >> 8) & 0xFF) as u8,
        (memory_address & 0xFF) as u8,
        value
    ];

    i2c.write(address, &buff).unwrap();
}

fn read_data(i2c: &mut hal::i2c::I2c<pac::I2C1, (stm32f3xx_hal_v2::gpio::gpiob::PB8<hal::gpio::AF4>, stm32f3xx_hal_v2::gpio::gpiob::PB9<hal::gpio::AF4>)>, address: u8, memory_address: u16, data: &mut [u8]) {
    let memory_address_bytes = [(memory_address >> 8) as u8, memory_address as u8];
    i2c.write_read(address, &memory_address_bytes, data).unwrap();
}


#[cfg(debug_assertions)]
fn checkpoint(){

    unsafe {
        asm!(
            "add sp, #280"
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
            "sub sp, #280"
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
            "add r0, #288",
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r13_sp
        );
    }
    
    //let dp = Peripherals::take().unwrap();

    unsafe{
    let dp = Peripherals::steal();

    let mut flash= dp.FLASH;
    unlock(& mut flash);
    wait_ready(&flash);

   
        //let  start_address: u32 = 0x2000_fffc as u32;
        let mut start_address:u32;
        let  end_address = r13_sp;
        asm!("movw r0, 0x9FF8
             movt r0, 0x2000");

         asm!(
             "MOV {0}, r0",
             out(reg) start_address
         );

         let stack_size = (start_address - end_address) + 4;
        // leaving first xyz K for program i.e start at 0x0801_0000
         let mut flash_start_address = Volatile::new(0x0803_0000);
         let mut flash_end_address = Volatile::new(0x0807_FFFF);    

        let mut checkpoint_size= Volatile::new(0u32);
        asm::dmb();
        checkpoint_size.write(stack_size+4+16*4 +4);
        asm::dmb();

        loop{
            let mut offset = ptr::read_volatile(flash_start_address.read() as *const u32);
            if offset == 0xffff_ffff{
                break;
            }
            flash_start_address.write(flash_start_address.read() + offset); 
            if flash_start_address.read() + checkpoint_size.read() >= flash_end_address.read(){
                erase_all(&mut flash);
               flash_start_address = Volatile::new(0x0801_0000);
            }
        }
        asm::dmb();
        //write the size of packet at the begining of the packet
        write_to_flash(&mut flash,  (flash_start_address.read()) as u32, checkpoint_size.read() as u32); 
        flash_start_address.write(flash_start_address.read()+4);
        asm::dmb();
           // Code that involves Flash write
    //      if offset == 0xffff_ffff {
    //   // stack_size + 4(0xffff_ffff to signal end of stack) + 16*4(store registers) + 4 (size of a packet)
    //         write_to_flash(&mut flash,  flash_start_address as u32, checkpoint_size+1-1  as u32);
    //         flash_start_address = flash_start_address + 4;
    //      }
    //      else{
    //         flash_start_address = flash_start_address + offset; 
    //         let flash_end_address = 0x0807_FFFF-1+1;
    //         if flash_end_address - flash_start_address < checkpoint_size {
    //             //clear flash
    //             //set start address and offset
    //             erase_all(&mut flash);
    //             flash_start_address = 0x0801_0004;
    //             offset = 0;
    //         }
    //         write_to_flash(&mut flash,  0x0801_0000 as u32, offset+checkpoint_size+1-1  as u32);
    //      }
    asm::dmb(); 
         while start_address >= end_address{
            let mut data = Volatile::new(0u32);
            data.write(core::ptr::read_volatile(start_address as * const u32));
            write_to_flash(&mut flash,  flash_start_address.read() as u32, data.read() as u32);
            flash_start_address.write(flash_start_address.read() +1* 4);
            // Move to the next address based on the size of the type
            start_address = start_address-4;
            
        }
        asm::dmb();
    asm::dmb();
    //mark the end of the stack
    write_to_flash(&mut flash,  (flash_start_address.read()) as u32, 0xffff_ffff as u32);
    flash_start_address.write(flash_start_address.read() + 4);
    asm::dmb();

    // for i in 0..15{
    //     write_to_flash(&mut flash,  0x0800_9060 as u32, r0_value as u32);
    //       flash_start_address = flash_start_address + 4;
    // }


    write_to_flash(&mut flash,  flash_start_address.read() as u32, r0_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+4 as u32, r1_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+8 as u32, r2_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+12 as u32, r3_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+16 as u32, r4_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+20 as u32, r5_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+24 as u32, r6_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+28 as u32, r7_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+32 as u32, r8_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+36 as u32, r9_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+40 as u32, r10_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+44 as u32, r11_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+48 as u32, r12_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+52 as u32, r13_sp as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+56 as u32, r14_lr as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+60 as u32, r15_pc as u32);
    drop(flash);
    }     
}

fn erase_all(flash: &mut FLASH){
    let start_address = 0x0801_0000;

    for i in 0..255{
        let page = start_address + i * 2*1024;
         erase_page(flash,  page);
    }

}
fn restore()->bool{
    unsafe {
        let mut flash_start_address = 0x0803_0000;
        let packet_size = ptr::read_volatile(0x0803_0000 as *const u32);
        //let r0_flash = ptr::read_volatile(0x0800_9060 as *const u32);
        if packet_size == 0xffff_ffff {
            return false
        }
        if  ptr::read_volatile((flash_start_address + packet_size) as *const u32)==0xffff_ffff{
            return  false;
        }

        let mut offset:u32 = 0;
        // think about multiple conditions where it could break
        //1. There could multiple failed checkpoints before a successful checkpoint.
        //2. The last checkpoint could be a failed(incomplete) checkpoint.
        loop{
            
            offset = ptr::read_volatile(flash_start_address  as *const u32);
             
            if  ptr::read_volatile((flash_start_address + offset) as *const u32) == 0xffff_ffff{
                break;
            }
    
            flash_start_address+=offset;
        }
        flash_start_address+=4;
       
        // let mut end_address = 0x0801_0004 + packet_size;
        // let recent_frame_size: u32 = ptr::read_volatile(end_address as *const u32);
        // let mut recent_frame_start_address = end_address - recent_frame_size;

        asm!(
            "mov r0, {0}",
            in(reg) flash_start_address
        );

        //set sp to 0x0200_fffc
        asm!("movw r1, 0x9ff8
        movt r1, 0x02000");
        asm!("msr msp, r1");

        asm!("movw r3, 0xffff
        movt r3, 0xffff");
    
        asm!("1:
            ldr r1, [r0, #4]
            cmp r1, r3
            beq 2f
            push {{r1}}
            adds r0, r0, #4
            b 1b
            2:");     

        asm!("adds r0, r0, #4");
        asm!("adds r0, r0, #4");

        asm!( "LDR r1, [r0]");
        asm!("Push {{r1}}");

        asm!("adds r0, r0, #4");
        asm!( "LDR r1, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r2, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r3, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r4, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r5, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r6, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r7, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r8, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r9, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r10, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r11, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r12, [r0]");

        asm!("adds r0, r0, #4");
       // asm!( "LDR r13, [r0]"); //no need to do this

        asm!("adds r0, r0, #4");
        asm!( "LDR r14, [r0]");

        asm!("POP {{r0}}");
        asm!("mov r15, r14");
    }
    return true;
}

fn test_checkpoint(){
    unsafe {
        asm!("mov r0, #10
              mov r1, #20
              mov r2, #30
              mov r3, #40
              mov r4, #50
              mov r5, #20
              mov r6, #30
              mov r7, #40
              mov r8, #50
        "); 
        }
    checkpoint();
    unsafe {
        asm!("add r0, r1"); 
        }
}

fn delete_pg(page: u32){
    unsafe{
    let mut dp = Peripherals::steal();
    let mut flash= &mut dp.FLASH;
    unlock(& mut flash); 
    wait_ready(&flash);
    erase_page(&mut flash,  page);
    }
}


fn restore_globals<T>(i2c: &mut hal::i2c::I2c<pac::I2C1, (stm32f3xx_hal_v2::gpio::gpiob::PB8<hal::gpio::AF4>, stm32f3xx_hal_v2::gpio::gpiob::PB9<hal::gpio::AF4>)>, adrs: *const T, len: usize) {
    unsafe {
        x  = 6;
        let mut data = [0u8; 20];
        i2c.write_read(FRAM_ADDRESS, &[(back_up_address >> 8) as u8, back_up_address as u8], &mut data).unwrap();
        
        let dp = adrs as *mut u8;
        ptr::copy_nonoverlapping(data.as_ptr(), dp, len);
        back_up_address= back_up_address +len as u16;
    }
}

fn checkpoint_globals<T>(i2c: &mut hal::i2c::I2c<pac::I2C1, (stm32f3xx_hal_v2::gpio::gpiob::PB8<hal::gpio::AF4>, stm32f3xx_hal_v2::gpio::gpiob::PB9<hal::gpio::AF4>)>, adrs: *const T, len: usize) {
    unsafe {
    let mut buff = [0;100];
    buff[0]= ((back_up_address >> 8) & 0xFF) as u8;
    buff[1] = (back_up_address & 0xFF) as u8;
    
    for i in 0..len{
       let byte_ptr = adrs as *const u8;
       buff[i+2] =  *byte_ptr.add(i);
    }

    i2c.write(FRAM_ADDRESS, &buff);

    back_up_address= back_up_address +len as u16;
    }
}

macro_rules! copy_slice {
    ($data:expr, $dp:expr, $len:expr) => {
        unsafe {
            ptr::copy_nonoverlapping($data.as_ptr(), $dp, $len);
        }
    };
}

macro_rules! restore_globals {
    ($first:expr, $mem_locs:expr, $sizes:expr) => {{
        // Allocate memory for the reading data array
        let mut buff = [0;50];
        unsafe {
            let mut data = [0u8; 50];
            $first.write_read(FRAM_ADDRESS, &[(back_up_address >> 8) as u8, back_up_address as u8], &mut data).unwrap();
            let mut step = 0;
            //hprintln!("read array {:?}", data);
            for (mem_loc, size) in $mem_locs.iter().zip($sizes.iter()) {
                //hprintln!("{:?}", [step..step+*size+1]);
                copy_slice!(&data[step..step+*size+1], *mem_loc as _, *size);    
                step = step + *size;
            }
    }
    }};
}

macro_rules! checkpoint_globals {
    ($first:expr, $mem_locs:expr, $sizes:expr) => {{
        // Calculate the combined size of all arrays
        let len: usize = $sizes.iter().sum();

        // Allocate memory for the backing up data in flash
        let mut buff = [0;50];

        buff[0]= ((back_up_address >> 8) & 0xFF) as u8;
        buff[1] = (back_up_address & 0xFF) as u8;
    
        // Iterate over memory locations and sizes simultaneously
        let mut step = 0;
        for (mem_loc, size) in $mem_locs.iter().zip($sizes.iter()) {
            // Copy elements of each array into the combined array
            let byte_ptr = *mem_loc as *const u8;    
            for i in 0..*size{
                buff[i+ step+2] =  *byte_ptr.add(i);   
            }
            step = step + *size;
        }
        //hprintln!("buff {:?}", buff);
        $first.write(FRAM_ADDRESS, &buff);

    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    //delete_pg(0x0803_0000 as u32);  //0x0807_F800
    let mut i2c = init_i2c();
    unsafe{
            // Define arrays for memory locations and sizes
            let mem_locs = [&y as *const u8, &z as *const u8];
            let sizes = [mem::size_of_val(&y), mem::size_of_val(&z)];
            checkpoint_globals!(&mut i2c, &mem_locs, &sizes);
            restore_globals!(&mut i2c, &mem_locs, &sizes);

    }
    unsafe{
        hprintln!("The value y {}", y).unwrap();
        hprintln!("The value z {}", z).unwrap();
    }

    // unsafe{
    //     restore_globals(&mut i2c, &y as *const u8, mem::size_of_val(&y));
    //     restore_globals(&mut i2c, &z as *const u8, mem::size_of_val(&z));
    // }

    // restore();
    unsafe{
        let c = y+z;
        //checkpoint_variables(&mut y, &mut z); // y and z are the exclusive may write variables
        //inline


        // checkpoint_globals(&mut i2c, &y as *const u8, mem::size_of_val(&y));
        // checkpoint_globals(&mut i2c, &z as *const u8, mem::size_of_val(&z));
        //checkpoint();

        let d = c *2;

        // t = 10;
        // if t>=5{

        //     i2c.write(FRAM_ADDRESS, &[((0x0000 >> 8) & 0xFF) as u8 ,(0x0000 & 0xFF) as u8, 6u8]);//x = 6;
            
        //     x = 6;
        //     i2c.write(FRAM_ADDRESS, &[((0x0001 >> 8) & 0xFF) as u8 ,(0x0001 & 0xFF) as u8, 7u8]);//y = 7;
        //     y = 7;
        // }else{
        //     let mut data = [0u8];
        //     i2c.write_read(FRAM_ADDRESS, &[(0x0002 >> 8) as u8, 0x0002 as u8], &mut data).unwrap();
        //     x = data[0];//x = z;
        //     z = 8;
        //     i2c.write(FRAM_ADDRESS, &[((0x0002 >> 8) & 0xFF) as u8 ,(0x0002 & 0xFF) as u8, 8u8]);
        // }
    }
  
    
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);

   loop {}
}
