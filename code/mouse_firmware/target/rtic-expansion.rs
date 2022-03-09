#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal :: otg_fs
    :: { UsbBus, USB } ; use stm32f4xx_hal :: prelude :: * ; use stm32f4xx_hal
    :: gpio :: * ; use usb_device :: { bus :: UsbBusAllocator, prelude :: * }
    ; use usbd_hid ::
    {
        descriptor :: { generator_prelude :: *, MouseReport }, hid_class ::
        HIDClass,
    } ; use crate :: hidDescriptors :: MouseKeyboard ; use crate ::
    mouseKeyboardReport :: MouseKeyboardState ; use crate :: bin ::
    macroSystem :: { MacroConfig, Function } ;
    #[doc = r" User code from within the module"] type Button = ErasedPin <
    Input < PullUp > > ; const POLL_INTERVAL_MS : u8 = 1 ;
    #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(cx : init :: Context) -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let mut dp = cx.device ; let
        cd = cx.core ; let rcc = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ; let gpioa =
        dp.GPIOA.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; let gpioc = dp.GPIOC.split() ; let mut button =
        gpioc.pc13.into_pull_up_input().erase() ; let mut sys_cfg =
        dp.SYSCFG.constrain() ; button.make_interrupt_source(& mut sys_cfg) ;
        button.enable_interrupt(& mut dp.EXTI) ;
        button.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        cx.local.bus.replace(UsbBus :: new(usb, cx.local.EP_MEMORY)) ; let hid
        = HIDClass ::
        new(cx.local.bus.as_ref().unwrap(), MouseKeyboard :: desc(),
            POLL_INTERVAL_MS,) ; let mouse = MouseKeyboardState :: new() ; let
        mut macro_conf = MacroConfig :: new() ;
        macro_conf.update_config(Function :: PressKeyboard(0x4), Function ::
                                 Nothing, Function :: Nothing, Function ::
                                 Nothing, Function :: Nothing, Function ::
                                 Nothing, Function :: Nothing) ; let usb_dev =
        UsbDeviceBuilder ::
        new(cx.local.bus.as_ref().unwrap(),
            UsbVidPid(0xc410,
                      0x0000)).manufacturer("e7020e").product("Mouse").serial_number("1337").device_class(0).build()
        ;
        (Shared { mouse, macro_conf }, Local { usb_dev, hid, button }, init ::
         Monotonics())
    } #[allow(non_snake_case)] fn
    button_pressed(mut cx : button_pressed :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.button.clear_interrupt_pending_bit() ; if
        cx.local.button.is_low()
        {
            rprintln! ("button low") ;
            cx.shared.mouse.lock(| mouse |
                                 {
                                     cx.shared.macro_conf.lock(| macro_conf |
                                                               {
                                                                   macro_conf.push_left(mouse)
                                                                   ;
                                                               }) ;
                                 }) ;
        } else
        {
            rprintln! ("button high") ;
            cx.shared.mouse.lock(| mouse |
                                 {
                                     cx.shared.macro_conf.lock(| macro_conf |
                                                               {
                                                                   macro_conf.release_left(mouse)
                                                                   ;
                                                               }) ;
                                 }) ;
        }
    } #[allow(non_snake_case)] fn usb_fs(mut cx : usb_fs :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let usb_fs
        :: LocalResources { usb_dev, hid, first, counter, } = cx.local ; if *
        first { rprintln! ("first") ; * first = false ; } * counter =
        (* counter + 1) % 200 ; let mov = match * counter
        {
            100 => { rprintln! ("10") ; 10 } 0 => { rprintln! ("-10") ; - 10 }
            _ => 0,
        } ; cx.shared.mouse.lock(| mouse | { mouse.add_x_movement(mov) ; }) ;
        cx.shared.mouse.lock(| mouse |
                             {
                                 let report = mouse.get_report_and_reset() ;
                                 hid.push_input(& report).ok() ;
                             }) ; if usb_dev.poll(& mut [hid]) { return ; }
    } struct Shared { mouse : MouseKeyboardState, macro_conf : MacroConfig, }
    struct Local
    {
        usb_dev : UsbDevice < 'static, UsbBus < USB > >, hid : HIDClass <
        'static, UsbBus < USB > >, button : Button,
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
    } mod shared_resources
    {
        use rtic :: export :: Priority ; #[doc(hidden)]
        #[allow(non_camel_case_types)] pub struct
        mouse_that_needs_to_be_locked < 'a > { priority : & 'a Priority, }
        impl < 'a > mouse_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { mouse_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        macro_conf_that_needs_to_be_locked < 'a >
        { priority : & 'a Priority, } impl < 'a >
        macro_conf_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { macro_conf_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `button_pressed` has access to"] pub struct
    __rtic_internal_button_pressedLocalResources < 'a >
    { pub button : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `button_pressed` has access to"] pub struct
    __rtic_internal_button_pressedSharedResources < 'a >
    {
        pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >,
        pub macro_conf : shared_resources ::
        macro_conf_that_needs_to_be_locked < 'a >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_button_pressed_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        button_pressed :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        button_pressed :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_button_pressed_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_button_pressed_Context
            {
                local : button_pressed :: LocalResources :: new(), shared :
                button_pressed :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod button_pressed
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_button_pressedLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_button_pressedSharedResources as SharedResources ; pub
        use super :: __rtic_internal_button_pressed_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `usb_fs` has access to"] pub struct
    __rtic_internal_usb_fsLocalResources < 'a >
    {
        pub usb_dev : & 'a mut UsbDevice < 'static, UsbBus < USB > >, pub hid
        : & 'a mut HIDClass < 'static, UsbBus < USB > >, pub first : & 'a mut
        bool, pub counter : & 'a mut u16,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `usb_fs` has access to"] pub struct
    __rtic_internal_usb_fsSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_usb_fs_Context <
    'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        usb_fs :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        usb_fs :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_usb_fs_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usb_fs_Context
            {
                local : usb_fs :: LocalResources :: new(), shared : usb_fs ::
                SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod usb_fs
    {
        #[doc(inline)] pub use super :: __rtic_internal_usb_fsLocalResources
        as LocalResources ; #[doc(inline)] pub use super ::
        __rtic_internal_usb_fsSharedResources as SharedResources ; pub use
        super :: __rtic_internal_usb_fs_Context as Context ;
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
    #[doc(hidden)] #[link_section = ".uninit.rtic195"] static
    __rtic_internal_shared_resource_mouse : rtic :: RacyCell < core :: mem ::
    MaybeUninit < MouseKeyboardState >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: mouse_that_needs_to_be_locked < 'a >
    {
        type T = MouseKeyboardState ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut MouseKeyboardState) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_mouse.get_mut() as * mut
                     _, self.priority(), CEILING, stm32f4 :: stm32f411 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic196"] static
    __rtic_internal_shared_resource_macro_conf : rtic :: RacyCell < core ::
    mem :: MaybeUninit < MacroConfig >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: macro_conf_that_needs_to_be_locked < 'a >
    {
        type T = MacroConfig ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut MacroConfig) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_macro_conf.get_mut() as *
                     mut _, self.priority(), CEILING, stm32f4 :: stm32f411 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic197"] static
    __rtic_internal_local_resource_usb_dev : rtic :: RacyCell < core :: mem ::
    MaybeUninit < UsbDevice < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic198"] static
    __rtic_internal_local_resource_hid : rtic :: RacyCell < core :: mem ::
    MaybeUninit < HIDClass < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic199"] static
    __rtic_internal_local_resource_button : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
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
    EXTI15_10()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                button_pressed(button_pressed :: Context ::
                               new(& rtic :: export :: Priority ::
                                   new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_button_pressedLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_button_pressedLocalResources
            {
                button : & mut *
                (& mut *
                 __rtic_internal_local_resource_button.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_button_pressedSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_button_pressedSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority), macro_conf : shared_resources ::
                macro_conf_that_needs_to_be_locked :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn OTG_FS()
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
    } impl < 'a > __rtic_internal_usb_fsSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usb_fsSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_send :: < MouseKeyboardState > () ; rtic
            :: export :: assert_send :: < MacroConfig > () ; rtic :: export ::
            assert_send :: < UsbDevice < 'static, UsbBus < USB > > > () ; rtic
            :: export :: assert_send :: < HIDClass < 'static, UsbBus < USB > >
            > () ; rtic :: export :: assert_send :: < Button > () ; rtic ::
            export :: interrupt :: disable() ; let mut core : rtic :: export
            :: Peripherals = rtic :: export :: Peripherals :: steal().into() ;
            let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: EXTI0 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI15_10, rtic :: export
                                   ::
                                   logical2hw(1u8, stm32f4 :: stm32f411 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI15_10) ; let _ =
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
                                      __rtic_internal_shared_resource_mouse.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(shared_resources.mouse))
                                      ;
                                      __rtic_internal_shared_resource_macro_conf.get_mut().write(core
                                                                                                 ::
                                                                                                 mem
                                                                                                 ::
                                                                                                 MaybeUninit
                                                                                                 ::
                                                                                                 new(shared_resources.macro_conf))
                                      ;
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
                                      ;
                                      __rtic_internal_local_resource_button.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(local_resources.button))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ; loop { rtic :: export :: nop() }
        }
    }
}