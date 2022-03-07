#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal :: otg_fs
    :: { UsbBus, USB } ; use stm32f4xx_hal :: prelude :: * ; use usb_device ::
    { bus :: UsbBusAllocator, prelude :: * } ; use usbd_hid ::
    {
        descriptor :: { generator_prelude :: *, MouseReport }, hid_class ::
        HIDClass,
    } ; #[doc = r" User code from within the module"] const POLL_INTERVAL_MS :
    u8 = 1 ; #[doc = r" User code end"] #[inline(always)]
    #[allow(non_snake_case)] fn init(cx : init :: Context) ->
    (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let dp = cx.device ; let rcc
        = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ; let gpioa =
        dp.GPIOA.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; cx.local.bus.replace(UsbBus :: new(usb, cx.local.EP_MEMORY)) ; let
        hid = HIDClass ::
        new(cx.local.bus.as_ref().unwrap(), MouseReport :: desc(),
            POLL_INTERVAL_MS,) ; let usb_dev = UsbDeviceBuilder ::
        new(cx.local.bus.as_ref().unwrap(),
            UsbVidPid(0xc410,
                      0x0000)).manufacturer("e7020e").product("Mouse").serial_number("1337").device_class(0).build()
        ; (Shared {}, Local { usb_dev, hid }, init :: Monotonics())
    } #[allow(non_snake_case)] fn usb_fs(cx : usb_fs :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let usb_fs
        :: LocalResources { usb_dev, hid, first, counter, } = cx.local ; if *
        first { rprintln! ("first") ; * first = false ; } * counter =
        (* counter + 1) % 200 ; let report = MouseReport
        {
            x : match * counter
            {
                100 => { rprintln! ("10") ; 10 } 0 =>
                { rprintln! ("-10") ; - 10 } _ => 0,
            }, y : 0, buttons : 0, wheel : 0, pan : 0,
        } ; hid.push_input(& report).ok() ; if usb_dev.poll(& mut [hid])
        { return ; }
    } struct Shared {} struct Local
    {
        usb_dev : UsbDevice < 'static, UsbBus < USB > >, hid : HIDClass <
        'static, UsbBus < USB > >,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < >
    {
        pub EP_MEMORY : & 'static mut [u32 ; 1024], pub bus : & 'static mut
        Option < UsbBusAllocator < UsbBus < USB > > >,
    } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
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
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `usb_fs` has access to"] pub struct
    __rtic_internal_usb_fsLocalResources < 'a >
    {
        pub usb_dev : & 'a mut UsbDevice < 'static, UsbBus < USB > >, pub hid
        : & 'a mut HIDClass < 'static, UsbBus < USB > >, pub first : & 'a mut
        bool, pub counter : & 'a mut u16,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_usb_fs_Context <
    'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        usb_fs :: LocalResources < 'a >,
    } impl < 'a > __rtic_internal_usb_fs_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usb_fs_Context
            { local : usb_fs :: LocalResources :: new(), }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod usb_fs
    {
        #[doc(inline)] pub use super :: __rtic_internal_usb_fsLocalResources
        as LocalResources ; pub use super :: __rtic_internal_usb_fs_Context as
        Context ;
    } #[doc = r" app module"] impl < > __rtic_internal_initLocalResources < >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            {
                EP_MEMORY : & mut *
                __rtic_internal_local_init_EP_MEMORY.get_mut(), bus : & mut *
                __rtic_internal_local_init_bus.get_mut(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_local_resource_usb_dev : rtic :: RacyCell < core :: mem ::
    MaybeUninit < UsbDevice < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_local_resource_hid : rtic :: RacyCell < core :: mem ::
    MaybeUninit < HIDClass < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_EP_MEMORY : rtic ::
    RacyCell < [u32 ; 1024] > = rtic :: RacyCell :: new([0 ; 1024]) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_bus : rtic :: RacyCell <
    Option < UsbBusAllocator < UsbBus < USB > > > > = rtic :: RacyCell ::
    new(None) ; #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_local_usb_fs_first : rtic :: RacyCell < bool > = rtic ::
    RacyCell :: new(true) ; #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_local_usb_fs_counter : rtic :: RacyCell < u16 > = rtic ::
    RacyCell :: new(0) ; #[allow(non_snake_case)] #[no_mangle] unsafe fn
    OTG_FS()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                usb_fs(usb_fs :: Context ::
                       new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_usb_fsLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_usb_fsLocalResources
            {
                usb_dev : & mut *
                (& mut *
                 __rtic_internal_local_resource_usb_dev.get_mut()).as_mut_ptr(),
                hid : & mut *
                (& mut *
                 __rtic_internal_local_resource_hid.get_mut()).as_mut_ptr(),
                first : & mut * __rtic_internal_local_usb_fs_first.get_mut(),
                counter : & mut *
                __rtic_internal_local_usb_fs_counter.get_mut(),
            }
        }
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_send :: < UsbDevice < 'static, UsbBus <
            USB > > > () ; rtic :: export :: assert_send :: < HIDClass <
            'static, UsbBus < USB > > > () ; rtic :: export :: interrupt ::
            disable() ; let mut core : rtic :: export :: Peripherals = rtic ::
            export :: Peripherals :: steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: EXTI0 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: OTG_FS, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f411 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: OTG_FS) ; #[inline(never)] fn
            __rtic_init_resources < F > (f : F) where F : FnOnce() { f() ; }
            __rtic_init_resources(||
                                  {
                                      let(shared_resources, local_resources,
                                          mut monotonics) =
                                      init(init :: Context ::
                                           new(core.into())) ;
                                      __rtic_internal_local_resource_usb_dev.get_mut().write(core
                                                                                             ::
                                                                                             mem
                                                                                             ::
                                                                                             MaybeUninit
                                                                                             ::
                                                                                             new(local_resources.usb_dev))
                                      ;
                                      __rtic_internal_local_resource_hid.get_mut().write(core
                                                                                         ::
                                                                                         mem
                                                                                         ::
                                                                                         MaybeUninit
                                                                                         ::
                                                                                         new(local_resources.hid))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ; loop { rtic :: export :: nop() }
        }
    }
}