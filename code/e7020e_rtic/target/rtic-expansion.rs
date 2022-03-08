#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    stm32f4xx_hal :: otg_fs :: { UsbBus, USB } ; use stm32f4xx_hal :: prelude
    :: * ; use usb_device :: prelude :: * ;
    #[doc = r" User code from within the module"] #[doc = r" User code end"]
    #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local, init :: Monotonics)
    {
        let dp = cx.device ; let rcc = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ; let gpioa =
        dp.GPIOA.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; let usb_bus = UsbBus :: new(usb, cx.local.EP_MEMORY) ; let mut
        serial = usbd_serial :: SerialPort :: new(& usb_bus) ; let mut usb_dev
        = UsbDeviceBuilder ::
        new(& usb_bus,
            UsbVidPid(0x16c0,
                      0x27dd)).manufacturer("e7020e").product("Serial port").serial_number("1337").device_class(usbd_serial
                                                                                                                ::
                                                                                                                USB_CLASS_CDC).build()
        ; loop
        {
            if! usb_dev.poll(& mut [& mut serial]) { continue ; } let mut buf
            = [0u8 ; 64] ; match serial.read(& mut buf)
            {
                Ok(count) if count > 0 =>
                {
                    for c in buf [0 .. count].iter_mut()
                    { if 0x61 <= * c && * c <= 0x7a { * c &=! 0x20 ; } } let
                    mut write_offset = 0 ; while write_offset < count
                    {
                        match serial.write(& buf [write_offset .. count])
                        {
                            Ok(len) if len > 0 => { write_offset += len ; } _
                            => {}
                        }
                    }
                } _ => {}
            }
        } (Shared {}, Local {}, init :: Monotonics())
    } struct Shared {} struct Local {} #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < >
    { pub EP_MEMORY : & 'static mut [u32 ; 1024], }
    #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_Monotonics() ;
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals"] pub device : stm32f4 ::
        stm32f411 :: Peripherals, #[doc = r" Critical section token for init"]
        pub cs : rtic :: export :: CriticalSection < 'a >,
        #[doc = r" Local Resources this task has access to"] pub local : init
        :: LocalResources < >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                device : stm32f4 :: stm32f411 :: Peripherals :: steal(), cs :
                rtic :: export :: CriticalSection :: new(), core, local : init
                :: LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_initLocalResources as
        LocalResources ; pub use super :: __rtic_internal_Monotonics as
        Monotonics ; pub use super :: __rtic_internal_init_Context as Context
        ;
    } #[doc = r" app module"] impl < > __rtic_internal_initLocalResources < >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            {
                EP_MEMORY : & mut *
                __rtic_internal_local_init_EP_MEMORY.get_mut(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_EP_MEMORY : rtic ::
    RacyCell < [u32 ; 1024] > = rtic :: RacyCell :: new([0 ; 1024]) ;
    #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: interrupt :: disable() ; let mut core : rtic ::
            export :: Peripherals = rtic :: export :: Peripherals ::
            steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: EXTI0 ; #[inline(never)] fn __rtic_init_resources < F
            > (f : F) where F : FnOnce() { f() ; }
            __rtic_init_resources(||
                                  {
                                      let(shared_resources, local_resources,
                                          mut monotonics) =
                                      init(init :: Context ::
                                           new(core.into())) ; rtic :: export
                                      :: interrupt :: enable() ;
                                  }) ; loop { rtic :: export :: nop() }
        }
    }
}