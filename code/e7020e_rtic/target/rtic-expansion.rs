#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f401 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal :: otg_fs
    :: { UsbBus, USB } ; use stm32f4xx_hal :: prelude :: * ; use stm32f4xx_hal
    :: gpio :: * ; use usb_device :: { bus :: UsbBusAllocator, prelude :: * }
    ; use usbd_hid ::
    {
        descriptor :: { generator_prelude :: *, MouseReport }, hid_class ::
        HIDClass,
    } ; use app :: mouseReport :: MouseState ;
    #[doc = r" User code from within the module"] type Button = ErasedPin <
    Input < PullDown > > ; const POLL_INTERVAL_MS : u8 = 1 ;
    #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(cx : init :: Context) -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let mut dp = cx.device ; let
        cd = cx.core ; let rcc = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ; let gpioa =
        dp.GPIOA.split() ; let gpiob = dp.GPIOB.split() ; let gpioc =
        dp.GPIOC.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; let mut left = gpiob.pb0.into_pull_down_input().erase() ; let mut
        right = gpiob.pb1.into_pull_down_input().erase() ; let mut middle =
        gpiob.pb12.into_pull_down_input().erase() ; let mut front =
        gpioc.pc5.into_pull_down_input().erase() ; let mut back =
        gpioc.pc4.into_pull_down_input().erase() ; let mut sys_cfg =
        dp.SYSCFG.constrain() ; left.make_interrupt_source(& mut sys_cfg) ;
        left.enable_interrupt(& mut dp.EXTI) ;
        left.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        right.make_interrupt_source(& mut sys_cfg) ;
        right.enable_interrupt(& mut dp.EXTI) ;
        right.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        middle.make_interrupt_source(& mut sys_cfg) ;
        middle.enable_interrupt(& mut dp.EXTI) ;
        middle.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        front.make_interrupt_source(& mut sys_cfg) ;
        front.enable_interrupt(& mut dp.EXTI) ;
        front.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        back.make_interrupt_source(& mut sys_cfg) ;
        back.enable_interrupt(& mut dp.EXTI) ;
        back.trigger_on_edge(& mut dp.EXTI, Edge :: RisingFalling) ;
        cx.local.bus.replace(UsbBus :: new(usb, cx.local.EP_MEMORY)) ; let hid
        = HIDClass ::
        new(cx.local.bus.as_ref().unwrap(), MouseReport :: desc(),
            POLL_INTERVAL_MS,) ; let mouse = MouseState :: new() ; let usb_dev
        = UsbDeviceBuilder ::
        new(cx.local.bus.as_ref().unwrap(),
            UsbVidPid(0xc410,
                      0x0000)).manufacturer("e7020e").product("Mouse").serial_number("1337").device_class(0).build()
        ;
        (Shared { mouse }, Local
         { usb_dev, hid, left, right, middle, front, back }, init ::
         Monotonics())
    } #[allow(non_snake_case)] fn middle_hand(mut cx : middle_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.middle.clear_interrupt_pending_bit() ; if
        cx.local.middle.is_low()
        {
            rprintln! ("middle low") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_middle() ; }) ;
        } else
        {
            rprintln! ("middle high") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_middle() ; }) ;
        }
    } #[allow(non_snake_case)] fn left_hand(mut cx : left_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.left.clear_interrupt_pending_bit() ; if
        cx.local.left.is_low()
        {
            rprintln! ("left low") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_left() ; }) ;
        } else
        {
            rprintln! ("left high") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_left() ; }) ;
        }
    } #[allow(non_snake_case)] fn right_hand(mut cx : right_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.right.clear_interrupt_pending_bit() ; if
        cx.local.right.is_low()
        {
            rprintln! ("right low") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_right() ; }) ;
        } else
        {
            rprintln! ("right high") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_right() ; }) ;
        }
    } #[allow(non_snake_case)] fn front_hand(mut cx : front_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.front.clear_interrupt_pending_bit() ; if
        cx.local.front.is_low()
        { rprintln! ("front low") ; cx.shared.mouse.lock(| mouse | {}) ; }
        else
        { rprintln! ("front high") ; cx.shared.mouse.lock(| mouse | {}) ; }
    } #[allow(non_snake_case)] fn back_hand(mut cx : back_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.back.clear_interrupt_pending_bit() ; if
        cx.local.back.is_low()
        { rprintln! ("back low") ; cx.shared.mouse.lock(| mouse | {}) ; } else
        { rprintln! ("back high") ; cx.shared.mouse.lock(| mouse | {}) ; }
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
    } struct Shared { mouse : MouseState, } struct Local
    {
        usb_dev : UsbDevice < 'static, UsbBus < USB > >, hid : HIDClass <
        'static, UsbBus < USB > >, left : Button, right : Button, middle :
        Button, front : Button, back : Button,
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
        stm32f401 :: Peripherals, #[doc = r" Critical section token for init"]
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
                device : stm32f4 :: stm32f401 :: Peripherals :: steal(), cs :
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
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `middle_hand` has access to"] pub struct
    __rtic_internal_middle_handLocalResources < 'a >
    { pub middle : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `middle_hand` has access to"] pub struct
    __rtic_internal_middle_handSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_middle_hand_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        middle_hand :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        middle_hand :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_middle_hand_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_middle_hand_Context
            {
                local : middle_hand :: LocalResources :: new(), shared :
                middle_hand :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod middle_hand
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_middle_handLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_middle_handSharedResources as SharedResources ; pub
        use super :: __rtic_internal_middle_hand_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `left_hand` has access to"] pub struct
    __rtic_internal_left_handLocalResources < 'a >
    { pub left : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `left_hand` has access to"] pub struct
    __rtic_internal_left_handSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_left_hand_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        left_hand :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        left_hand :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_left_hand_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_left_hand_Context
            {
                local : left_hand :: LocalResources :: new(), shared :
                left_hand :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod left_hand
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_left_handLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_left_handSharedResources as SharedResources ; pub use
        super :: __rtic_internal_left_hand_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `right_hand` has access to"] pub struct
    __rtic_internal_right_handLocalResources < 'a >
    { pub right : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `right_hand` has access to"] pub struct
    __rtic_internal_right_handSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_right_hand_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        right_hand :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        right_hand :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_right_hand_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_right_hand_Context
            {
                local : right_hand :: LocalResources :: new(), shared :
                right_hand :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod right_hand
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_right_handLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_right_handSharedResources as SharedResources ; pub use
        super :: __rtic_internal_right_hand_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `front_hand` has access to"] pub struct
    __rtic_internal_front_handLocalResources < 'a >
    { pub front : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `front_hand` has access to"] pub struct
    __rtic_internal_front_handSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_front_hand_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        front_hand :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        front_hand :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_front_hand_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_front_hand_Context
            {
                local : front_hand :: LocalResources :: new(), shared :
                front_hand :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod front_hand
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_front_handLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_front_handSharedResources as SharedResources ; pub use
        super :: __rtic_internal_front_hand_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `back_hand` has access to"] pub struct
    __rtic_internal_back_handLocalResources < 'a >
    { pub back : & 'a mut Button, } #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = "Shared resources `back_hand` has access to"] pub struct
    __rtic_internal_back_handSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_back_hand_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        back_hand :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        back_hand :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_back_hand_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_back_hand_Context
            {
                local : back_hand :: LocalResources :: new(), shared :
                back_hand :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod back_hand
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_back_handLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_back_handSharedResources as SharedResources ; pub use
        super :: __rtic_internal_back_hand_Context as Context ;
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
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_mouse : rtic :: RacyCell < core :: mem ::
    MaybeUninit < MouseState >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: mouse_that_needs_to_be_locked < 'a >
    {
        type T = MouseState ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut MouseState) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_mouse.get_mut() as * mut
                     _, self.priority(), CEILING, stm32f4 :: stm32f401 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_local_resource_usb_dev : rtic :: RacyCell < core :: mem ::
    MaybeUninit < UsbDevice < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_local_resource_hid : rtic :: RacyCell < core :: mem ::
    MaybeUninit < HIDClass < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_left : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic4"] static
    __rtic_internal_local_resource_right : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic5"] static
    __rtic_internal_local_resource_middle : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic6"] static
    __rtic_internal_local_resource_front : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic7"] static
    __rtic_internal_local_resource_back : rtic :: RacyCell < core :: mem ::
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
                middle_hand(middle_hand :: Context ::
                            new(& rtic :: export :: Priority ::
                                new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_middle_handLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_middle_handLocalResources
            {
                middle : & mut *
                (& mut *
                 __rtic_internal_local_resource_middle.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_middle_handSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_middle_handSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI0()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                left_hand(left_hand :: Context ::
                          new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_left_handLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_left_handLocalResources
            {
                left : & mut *
                (& mut *
                 __rtic_internal_local_resource_left.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_left_handSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_left_handSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI1()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                right_hand(right_hand :: Context ::
                           new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_right_handLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_right_handLocalResources
            {
                right : & mut *
                (& mut *
                 __rtic_internal_local_resource_right.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_right_handSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_right_handSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI9_5()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                front_hand(front_hand :: Context ::
                           new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_front_handLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_front_handLocalResources
            {
                front : & mut *
                (& mut *
                 __rtic_internal_local_resource_front.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_front_handSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_front_handSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI4()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                back_hand(back_hand :: Context ::
                          new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_back_handLocalResources < 'a >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_back_handLocalResources
            {
                back : & mut *
                (& mut *
                 __rtic_internal_local_resource_back.get_mut()).as_mut_ptr(),
            }
        }
    } impl < 'a > __rtic_internal_back_handSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_back_handSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
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
            rtic :: export :: assert_send :: < MouseState > () ; rtic ::
            export :: assert_send :: < UsbDevice < 'static, UsbBus < USB > > >
            () ; rtic :: export :: assert_send :: < HIDClass < 'static, UsbBus
            < USB > > > () ; rtic :: export :: assert_send :: < Button > () ;
            rtic :: export :: interrupt :: disable() ; let mut core : rtic ::
            export :: Peripherals = rtic :: export :: Peripherals ::
            steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: DMA1_STREAM0 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI15_10, rtic :: export
                                   ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI15_10) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI0, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI0) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI1, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI1) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI9_5, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI9_5) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI4, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI4) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: OTG_FS, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
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
                                      __rtic_internal_local_resource_left.get_mut().write(core
                                                                                          ::
                                                                                          mem
                                                                                          ::
                                                                                          MaybeUninit
                                                                                          ::
                                                                                          new(local_resources.left))
                                      ;
                                      __rtic_internal_local_resource_right.get_mut().write(core
                                                                                           ::
                                                                                           mem
                                                                                           ::
                                                                                           MaybeUninit
                                                                                           ::
                                                                                           new(local_resources.right))
                                      ;
                                      __rtic_internal_local_resource_middle.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(local_resources.middle))
                                      ;
                                      __rtic_internal_local_resource_front.get_mut().write(core
                                                                                           ::
                                                                                           mem
                                                                                           ::
                                                                                           MaybeUninit
                                                                                           ::
                                                                                           new(local_resources.front))
                                      ;
                                      __rtic_internal_local_resource_back.get_mut().write(core
                                                                                          ::
                                                                                          mem
                                                                                          ::
                                                                                          MaybeUninit
                                                                                          ::
                                                                                          new(local_resources.back))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ; loop { rtic :: export :: nop() }
        }
    }
}