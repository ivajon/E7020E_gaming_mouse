#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f401 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; pub use
    rtic :: Monotonic as _ ;
    #[doc = r" Holds static methods for each monotonic."] pub mod monotonics
    {
        pub use MyMono :: now ;
        #[doc =
          "This module holds the static implementation for `MyMono::now()`"]
        #[allow(non_snake_case)] pub mod MyMono
        {
            #[doc = r" Read the current time from this monotonic"] pub fn
            now() -> < super :: super :: MyMono as rtic :: Monotonic > ::
            Instant
            {
                rtic :: export :: interrupt ::
                free(| _ |
                     {
                         use rtic :: Monotonic as _ ; if let Some(m) = unsafe
                         {
                             & mut * super :: super ::
                             __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()
                         } { m.now() } else
                         {
                             < super :: super :: MyMono as rtic :: Monotonic >
                             :: zero()
                         }
                     })
            }
        }
    } use app :: pmw3389 :: Pmw3389 ; use app :: mouseReport :: MouseState ;
    use app :: hidDescriptors :: MouseKeyboard ; use app ::
    mouseKeyboardReport :: MouseKeyboardState ; use eeprom :: EEPROM ; use
    stm32f4 :: stm32f401 :: * ; use rtt_target :: { rprintln, rtt_init_print }
    ; use stm32f4xx_hal :: otg_fs :: { UsbBus, USB } ; use stm32f4xx_hal ::
    prelude :: * ; use stm32f4xx_hal :: gpio :: * ; use dwt_systick_monotonic
    :: * ; use embedded_hal :: spi :: MODE_3 ; use usb_device ::
    { bus :: UsbBusAllocator, prelude :: * } ; use usbd_hid ::
    {
        descriptor :: { generator_prelude :: *, MouseReport }, hid_class ::
        HIDClass,
    } ; use stm32f4xx_hal ::
    {
        gpio :: { Alternate, Output, Pin, PushPull, Speed }, prelude :: *, spi
        :: { Spi, TransferModeNormal }, timer :: Delay,
    } ; use stm32f4 :: stm32f401 :: { SPI1, TIM5 } ;
    #[doc = r" User code from within the module"] const FREQ_CORE : u32 =
    16_000_000 ; type MyMono = DwtSystick < FREQ_CORE > ; type Button =
    ErasedPin < Input < PullDown > > ; type SCK = Pin < Alternate < PushPull,
    5_u8 >, 'A', 5_u8 > ; type MOSI = Pin < Alternate < PushPull, 5_u8 >, 'A',
    7_u8 > ; type MISO = Pin < Alternate < PushPull, 5_u8 >, 'A', 6_u8 > ;
    type CS = Pin < Output < PushPull >, 'A', 4_u8 > ; type SPI = Spi < SPI1,
    (SCK, MISO, MOSI), TransferModeNormal > ; type DELAY = Delay < TIM5,
    1000000_u32 > ; type PMW3389 = Pmw3389 < SPI, CS, DELAY > ; const
    POLL_INTERVAL_MS : u8 = 1 ; fn delay(td : u32)
    {
        let time = monotonics :: now().ticks() as u32 ; #[no_mangle]
        while(monotonics :: now().ticks() as u32 - time) < td { }
    } #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(cx : init :: Context) -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let mut cd = cx.core ; let
        mut dp = cx.device ; let mut sys_cfg = dp.SYSCFG.constrain() ; let
        mono = DwtSystick :: new(& mut cd.DCB, cd.DWT, cd.SYST, FREQ_CORE) ;
        let rcc = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze() ; let gpioa =
        dp.GPIOA.split() ; let gpiob = dp.GPIOB.split() ; let gpioc =
        dp.GPIOC.split() ; let usb = USB
        {
            usb_global : dp.OTG_FS_GLOBAL, usb_device : dp.OTG_FS_DEVICE,
            usb_pwrclk : dp.OTG_FS_PWRCLK, pin_dm :
            gpioa.pa11.into_alternate(), pin_dp : gpioa.pa12.into_alternate(),
            hclk : clocks.hclk(),
        } ; let sck : SCK =
        gpioa.pa5.into_alternate().set_speed(Speed :: VeryHigh) ; let miso :
        MISO = gpioa.pa6.into_alternate().set_speed(Speed :: High) ; let mosi
        : MOSI = gpioa.pa7.into_alternate().set_speed(Speed :: High) ; let cs
        : CS = gpioa.pa4.into_push_pull_output().set_speed(Speed :: High) ;
        let spi : SPI = Spi ::
        new(dp.SPI1, (sck, miso, mosi), MODE_3, 1.MHz(), & clocks) ; let delay
        : DELAY = dp.TIM5.delay_us(& clocks) ; let mut pmw3389 : PMW3389 =
        Pmw3389 :: new(spi, cs, delay).unwrap() ; pmw3389.store_cpi().ok() ;
        let mut motion = gpiob.pb13.into_pull_down_input().erase() ; let mut
        phase_a = gpiob.pb2.into_pull_down_input().erase() ; let mut phase_b =
        gpioc.pc9.into_pull_down_input().erase() ; let mut left =
        gpiob.pb0.into_pull_down_input().erase() ; let mut right =
        gpiob.pb1.into_pull_down_input().erase() ; let mut middle =
        gpiob.pb12.into_pull_down_input().erase() ; let mut front =
        gpioc.pc5.into_pull_down_input().erase() ; let mut back =
        gpioc.pc4.into_pull_down_input().erase() ;
        phase_a.make_interrupt_source(& mut sys_cfg) ;
        phase_b.make_interrupt_source(& mut sys_cfg) ;
        phase_a.enable_interrupt(& mut dp.EXTI) ;
        phase_b.enable_interrupt(& mut dp.EXTI) ;
        phase_a.trigger_on_edge(& mut dp.EXTI, Edge :: Rising) ;
        phase_b.trigger_on_edge(& mut dp.EXTI, Edge :: Rising) ;
        left.make_interrupt_source(& mut sys_cfg) ;
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
        new(cx.local.bus.as_ref().unwrap(), MouseKeyboard :: desc(),
            POLL_INTERVAL_MS,) ; let mouse = MouseKeyboardState ::
        new(pmw3389) ; let usb_dev = UsbDeviceBuilder ::
        new(cx.local.bus.as_ref().unwrap(),
            UsbVidPid(0xc410,
                      0x1234)).manufacturer("Ivar och Erik").product("Banger gaming mus").serial_number("1234").device_class(0).build()
        ; usb_dev.remote_wakeup_enabled() ; let mut EXTI = dp.EXTI ; let ts =
        0 ;
        (Shared { mouse, EXTI }, Local
         {
             usb_dev, hid, left, right, middle, front, back, phase_a, phase_b,
             motion, ts
         }, init :: Monotonics(mono))
    } #[allow(non_snake_case)] fn idle(mut cx : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; loop
        { cx.shared.mouse.lock(| mouse | { mouse.read_sensor() ; }) ; }
    } #[allow(non_snake_case)] fn middle_hand(mut cx : middle_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.middle.clear_interrupt_pending_bit() ; if
        cx.local.middle.is_low()
        {
            rprintln! ("middle low") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_middle() ; }) ;
        } else if cx.local.middle.is_high()
        {
            rprintln! ("middle high") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_middle() ; }) ;
        }
    } #[allow(non_snake_case)] fn left_hand(mut cx : left_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.left.clear_interrupt_pending_bit() ; if
        cx.local.left.is_low()
        {
            rprintln! ("left low") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_left() ; }) ;
        } else
        {
            rprintln! ("left high") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_left() ; }) ;
        }
    } #[allow(non_snake_case)] fn right_hand(mut cx : right_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.right.clear_interrupt_pending_bit() ; if
        cx.local.right.is_low()
        {
            rprintln! ("right low") ;
            cx.shared.mouse.lock(| mouse | { mouse.release_right() ; }) ;
        } else
        {
            rprintln! ("right high") ;
            cx.shared.mouse.lock(| mouse | { mouse.push_right() ; }) ;
        }
    } #[allow(non_snake_case)] fn front_hand(mut cx : front_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.front.clear_interrupt_pending_bit() ;
        cx.shared.EXTI.lock(| EXTI |
                            { cx.local.front.disable_interrupt(EXTI) ; }) ;
        delay(160000) ;
        cx.shared.EXTI.lock(| EXTI |
                            { cx.local.front.enable_interrupt(EXTI) ; }) ; if
        cx.local.front.is_low()
        { rprintln! ("front low") ; cx.shared.mouse.lock(| mouse | { }) ; }
        else
        {
            rprintln! ("front high") ;
            cx.shared.mouse.lock(| mouse | { mouse.increment_dpi(1) ; }) ;
        }
    } #[allow(non_snake_case)] fn back_hand(mut cx : back_hand :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.local.back.clear_interrupt_pending_bit() ;
        cx.shared.EXTI.lock(| EXTI |
                            { cx.local.back.disable_interrupt(EXTI) ; }) ;
        delay(160000) ;
        cx.shared.EXTI.lock(| EXTI |
                            { cx.local.back.enable_interrupt(EXTI) ; }) ; if
        cx.local.back.is_low()
        {
            rprintln! ("back low") ;
            cx.shared.mouse.lock(| mouse | { mouse.increment_dpi(- 1) ; }) ;
        } else { cx.shared.mouse.lock(| mouse | { }) ; }
    } #[allow(non_snake_case)] fn usb_fs(mut cx : usb_fs :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let usb_fs
        :: LocalResources { usb_dev, hid, first, counter, } = cx.local ; if *
        first { rprintln! ("first") ; * first = false ; } let mut buf =
        [0u8 ; 1024] ; match hid.pull_raw_output(& mut buf).ok()
        {
            Some(len) =>
            {
                rprintln! ("{:?}", buf) ; handle_host_call ::
                spawn(buf).unwrap() ;
            }, None => { }
        }
        cx.shared.mouse.lock(| mouse |
                             {
                                 let report = mouse.get_report_and_reset() ;
                                 hid.push_input(& report).ok() ;
                             }) ; if usb_dev.poll(& mut [hid]) { return ; }
    } #[allow(non_snake_case)] fn
    handle_host_call(mut cx : handle_host_call :: Context, buffer :
                     [u8 ; 1024])
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; rprintln!
        ("handle host call") ; rprintln! ("{:?}", buffer) ; match buffer [0]
        {
            0x01 => { rprintln! ("RGB _controll") ; }, 0x02 =>
            {
                rprintln! ("DPI _controll") ; let dpi = (buffer [1] as u16) <<
                8 | buffer [2] as u16 ; handle_dpi :: spawn(dpi).unwrap() ;
            }, 0x03 => { rprintln! ("DPI _controll") ; }, 0x04 =>
            { rprintln! ("Macro _controll") ; }, _ =>
            { rprintln! ("unknown") ; }
        }
    } #[allow(non_snake_case)] fn
    handle_dpi(mut cx : handle_dpi :: Context, dpi : u16)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ;
        cx.shared.mouse.lock(| mouse | { mouse.write_dpi(dpi) ; }) ;
    } struct Shared { mouse : MouseKeyboardState, EXTI : EXTI, } struct Local
    {
        usb_dev : UsbDevice < 'static, UsbBus < USB > >, hid : HIDClass <
        'static, UsbBus < USB > >, left : Button, right : Button, middle :
        Button, front : Button, back : Button, phase_a : Button, phase_b :
        Button, motion : Button, ts : u32,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < >
    {
        pub EP_MEMORY : & 'static mut [u32 ; 1024], pub bus : & 'static mut
        Option < UsbBusAllocator < UsbBus < USB > > >,
    } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_Monotonics(pub DwtSystick < FREQ_CORE >) ;
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
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `idle` has access to"] pub struct
    __rtic_internal_idleSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        idle :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idle_Context
            { shared : idle :: SharedResources :: new(priority), }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleSharedResources as
        SharedResources ; pub use super :: __rtic_internal_idle_Context as
        Context ;
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
        EXTI_that_needs_to_be_locked < 'a > { priority : & 'a Priority, } impl
        < 'a > EXTI_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { EXTI_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `middle_hand` has access to"] pub struct
    __rtic_internal_middle_handLocalResources < 'a >
    {
        pub middle : & 'a mut Button, pub motion : & 'a mut Button, pub ts : &
        'a mut u32,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
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
    {
        pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >,
        pub EXTI : shared_resources :: EXTI_that_needs_to_be_locked < 'a >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
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
    {
        pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >,
        pub EXTI : shared_resources :: EXTI_that_needs_to_be_locked < 'a >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
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
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_handle_host_call_Context < > { } impl < >
    __rtic_internal_handle_host_call_Context < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        { __rtic_internal_handle_host_call_Context { } }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_handle_host_call_spawn(_0 : [u8 ; 1024],) -> Result < (),
    [u8 ; 1024] >
    {
        let input = _0 ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut *
                  __rtic_internal_handle_host_call_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_handle_host_call_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                                      ::
                                                                                      from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              handle_host_call,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f401 :: interrupt :: DMA1_STREAM0) ;
                Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_handle_host_call_MyMono_SpawnHandle
    { #[doc(hidden)] marker : u32, } impl core :: fmt :: Debug for
    __rtic_internal_handle_host_call_MyMono_SpawnHandle
    {
        fn fmt(& self, f : & mut core :: fmt :: Formatter < '_ >) -> core ::
        fmt :: Result { f.debug_struct("MyMono::SpawnHandle").finish() }
    } impl __rtic_internal_handle_host_call_MyMono_SpawnHandle
    {
        pub fn cancel(self) -> Result < [u8 ; 1024], () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let tq = & mut * __rtic_internal_TQ_MyMono.get_mut() ; if
                     let Some((_task, index)) = tq.cancel_marker(self.marker)
                     {
                         let msg =
                         (& *
                          __rtic_internal_handle_host_call_INPUTS.get()).get_unchecked(usize
                                                                                       ::
                                                                                       from(index)).as_ptr().read()
                         ;
                         (& mut *
                          __rtic_internal_handle_host_call_FQ.get_mut()).split().0.enqueue_unchecked(index)
                         ; Ok(msg)
                     } else { Err(()) }
                 })
        } #[inline] pub fn
        reschedule_after(self, duration : < MyMono as rtic :: Monotonic > ::
                         Duration) -> Result < Self, () >
        { self.reschedule_at(monotonics :: MyMono :: now() + duration) } pub
        fn
        reschedule_at(self, instant : < MyMono as rtic :: Monotonic > ::
                      Instant) -> Result < Self, () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let marker =
                     __rtic_internal_TIMER_QUEUE_MARKER.get().read() ;
                     __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(marker.wrapping_add(1))
                     ; let tq = (& mut * __rtic_internal_TQ_MyMono.get_mut())
                     ;
                     tq.update_marker(self.marker, marker, instant, || rtic ::
                                      export :: SCB ::
                                      set_pendst()).map(| _ | handle_host_call
                                                        :: MyMono ::
                                                        SpawnHandle
                                                        { marker })
                 })
        }
    }
    #[doc =
      r" Spawns the task after a set duration relative to the current time"]
    #[doc = r""]
    #[doc =
      r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
    #[doc =
      r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_handle_host_call_MyMono_spawn_after(duration : < MyMono as
                                                        rtic :: Monotonic > ::
                                                        Duration, _0 :
                                                        [u8 ; 1024]) -> Result
    < handle_host_call :: MyMono :: SpawnHandle, [u8 ; 1024] >
    {
        let instant = monotonics :: MyMono :: now() ;
        __rtic_internal_handle_host_call_MyMono_spawn_at(instant + duration,
                                                         _0)
    } #[doc = r" Spawns the task at a fixed time instant"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_handle_host_call_MyMono_spawn_at(instant : < MyMono as
                                                     rtic :: Monotonic > ::
                                                     Instant, _0 :
                                                     [u8 ; 1024]) -> Result <
    handle_host_call :: MyMono :: SpawnHandle, [u8 ; 1024] >
    {
        unsafe
        {
            let input = _0 ; if let Some(index) = rtic :: export :: interrupt
            ::
            free(| _ |
                 (& mut *
                  __rtic_internal_handle_host_call_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_handle_host_call_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                                      ::
                                                                                      from(index)).as_mut_ptr().write(input)
                ;
                (& mut *
                 __rtic_internal_handle_host_call_MyMono_INSTANTS.get_mut()).get_unchecked_mut(usize
                                                                                               ::
                                                                                               from(index)).as_mut_ptr().write(instant)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         let marker =
                         __rtic_internal_TIMER_QUEUE_MARKER.get().read() ; let
                         nr = rtic :: export :: NotReady
                         {
                             instant, index, task : SCHED_T ::
                             handle_host_call, marker,
                         } ;
                         __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(__rtic_internal_TIMER_QUEUE_MARKER.get().read().wrapping_add(1))
                         ; let tq = & mut *
                         __rtic_internal_TQ_MyMono.get_mut() ;
                         tq.enqueue_unchecked(nr, || core :: mem :: transmute
                                              :: < _, rtic :: export :: SYST >
                                              (()).enable_interrupt(), || rtic
                                              :: export :: SCB ::
                                              set_pendst(),
                                              (& mut *
                                               __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut())
                         ;
                         Ok(handle_host_call :: MyMono :: SpawnHandle
                            { marker })
                     })
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod
    handle_host_call
    {
        pub use super :: __rtic_internal_handle_host_call_Context as Context ;
        pub use super :: __rtic_internal_handle_host_call_spawn as spawn ; pub
        use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ; pub mod MyMono
        {
            pub use super :: super ::
            __rtic_internal_handle_host_call_MyMono_spawn_after as spawn_after
            ; pub use super :: super ::
            __rtic_internal_handle_host_call_MyMono_spawn_at as spawn_at ; pub
            use super :: super ::
            __rtic_internal_handle_host_call_MyMono_SpawnHandle as SpawnHandle
            ;
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `handle_dpi` has access to"] pub struct
    __rtic_internal_handle_dpiSharedResources < 'a >
    { pub mouse : shared_resources :: mouse_that_needs_to_be_locked < 'a >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_handle_dpi_Context < 'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        handle_dpi :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_handle_dpi_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_handle_dpi_Context
            { shared : handle_dpi :: SharedResources :: new(priority), }
        }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_handle_dpi_spawn(_0 : u16,) -> Result < (), u16 >
    {
        let input = _0 ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_handle_dpi_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_handle_dpi_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                                ::
                                                                                from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              handle_dpi,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f401 :: interrupt :: DMA1_STREAM0) ;
                Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_handle_dpi_MyMono_SpawnHandle
    { #[doc(hidden)] marker : u32, } impl core :: fmt :: Debug for
    __rtic_internal_handle_dpi_MyMono_SpawnHandle
    {
        fn fmt(& self, f : & mut core :: fmt :: Formatter < '_ >) -> core ::
        fmt :: Result { f.debug_struct("MyMono::SpawnHandle").finish() }
    } impl __rtic_internal_handle_dpi_MyMono_SpawnHandle
    {
        pub fn cancel(self) -> Result < u16, () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let tq = & mut * __rtic_internal_TQ_MyMono.get_mut() ; if
                     let Some((_task, index)) = tq.cancel_marker(self.marker)
                     {
                         let msg =
                         (& *
                          __rtic_internal_handle_dpi_INPUTS.get()).get_unchecked(usize
                                                                                 ::
                                                                                 from(index)).as_ptr().read()
                         ;
                         (& mut *
                          __rtic_internal_handle_dpi_FQ.get_mut()).split().0.enqueue_unchecked(index)
                         ; Ok(msg)
                     } else { Err(()) }
                 })
        } #[inline] pub fn
        reschedule_after(self, duration : < MyMono as rtic :: Monotonic > ::
                         Duration) -> Result < Self, () >
        { self.reschedule_at(monotonics :: MyMono :: now() + duration) } pub
        fn
        reschedule_at(self, instant : < MyMono as rtic :: Monotonic > ::
                      Instant) -> Result < Self, () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let marker =
                     __rtic_internal_TIMER_QUEUE_MARKER.get().read() ;
                     __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(marker.wrapping_add(1))
                     ; let tq = (& mut * __rtic_internal_TQ_MyMono.get_mut())
                     ;
                     tq.update_marker(self.marker, marker, instant, || rtic ::
                                      export :: SCB ::
                                      set_pendst()).map(| _ | handle_dpi ::
                                                        MyMono :: SpawnHandle
                                                        { marker })
                 })
        }
    }
    #[doc =
      r" Spawns the task after a set duration relative to the current time"]
    #[doc = r""]
    #[doc =
      r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
    #[doc =
      r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_handle_dpi_MyMono_spawn_after(duration : < MyMono as rtic
                                                  :: Monotonic > :: Duration,
                                                  _0 : u16) -> Result <
    handle_dpi :: MyMono :: SpawnHandle, u16 >
    {
        let instant = monotonics :: MyMono :: now() ;
        __rtic_internal_handle_dpi_MyMono_spawn_at(instant + duration, _0)
    } #[doc = r" Spawns the task at a fixed time instant"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_handle_dpi_MyMono_spawn_at(instant : < MyMono as rtic ::
                                               Monotonic > :: Instant, _0 :
                                               u16) -> Result < handle_dpi ::
    MyMono :: SpawnHandle, u16 >
    {
        unsafe
        {
            let input = _0 ; if let Some(index) = rtic :: export :: interrupt
            ::
            free(| _ |
                 (& mut * __rtic_internal_handle_dpi_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_handle_dpi_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                                ::
                                                                                from(index)).as_mut_ptr().write(input)
                ;
                (& mut *
                 __rtic_internal_handle_dpi_MyMono_INSTANTS.get_mut()).get_unchecked_mut(usize
                                                                                         ::
                                                                                         from(index)).as_mut_ptr().write(instant)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         let marker =
                         __rtic_internal_TIMER_QUEUE_MARKER.get().read() ; let
                         nr = rtic :: export :: NotReady
                         {
                             instant, index, task : SCHED_T :: handle_dpi,
                             marker,
                         } ;
                         __rtic_internal_TIMER_QUEUE_MARKER.get_mut().write(__rtic_internal_TIMER_QUEUE_MARKER.get().read().wrapping_add(1))
                         ; let tq = & mut *
                         __rtic_internal_TQ_MyMono.get_mut() ;
                         tq.enqueue_unchecked(nr, || core :: mem :: transmute
                                              :: < _, rtic :: export :: SYST >
                                              (()).enable_interrupt(), || rtic
                                              :: export :: SCB ::
                                              set_pendst(),
                                              (& mut *
                                               __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut())
                         ; Ok(handle_dpi :: MyMono :: SpawnHandle { marker })
                     })
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod handle_dpi
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_handle_dpiSharedResources as SharedResources ; pub use
        super :: __rtic_internal_handle_dpi_Context as Context ; pub use super
        :: __rtic_internal_handle_dpi_spawn as spawn ; pub use MyMono ::
        spawn_after ; pub use MyMono :: spawn_at ; pub use MyMono ::
        SpawnHandle ; pub mod MyMono
        {
            pub use super :: super ::
            __rtic_internal_handle_dpi_MyMono_spawn_after as spawn_after ; pub
            use super :: super :: __rtic_internal_handle_dpi_MyMono_spawn_at
            as spawn_at ; pub use super :: super ::
            __rtic_internal_handle_dpi_MyMono_SpawnHandle as SpawnHandle ;
        }
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
    } impl < 'a > __rtic_internal_idleSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idleSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
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
            #[doc = r" Priority ceiling"] const CEILING : u8 = 2u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_mouse.get_mut() as * mut
                     _, self.priority(), CEILING, stm32f4 :: stm32f401 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_EXTI : rtic :: RacyCell < core :: mem ::
    MaybeUninit < EXTI >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: EXTI_that_needs_to_be_locked < 'a >
    {
        type T = EXTI ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut EXTI) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_EXTI.get_mut() as * mut
                     _, self.priority(), CEILING, stm32f4 :: stm32f401 ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_local_resource_usb_dev : rtic :: RacyCell < core :: mem ::
    MaybeUninit < UsbDevice < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_hid : rtic :: RacyCell < core :: mem ::
    MaybeUninit < HIDClass < 'static, UsbBus < USB > > >> = rtic :: RacyCell
    :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic4"] static
    __rtic_internal_local_resource_left : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic5"] static
    __rtic_internal_local_resource_right : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic6"] static
    __rtic_internal_local_resource_middle : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic7"] static
    __rtic_internal_local_resource_front : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic8"] static
    __rtic_internal_local_resource_back : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic9"] static
    __rtic_internal_local_resource_phase_a : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic10"] static
    __rtic_internal_local_resource_phase_b : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic11"] static
    __rtic_internal_local_resource_motion : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Button >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic12"] static
    __rtic_internal_local_resource_ts : rtic :: RacyCell < core :: mem ::
    MaybeUninit < u32 >> = rtic :: RacyCell ::
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
        const PRIORITY : u8 = 2u8 ; rtic :: export ::
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
                motion : & mut *
                (& mut *
                 __rtic_internal_local_resource_motion.get_mut()).as_mut_ptr(),
                ts : & mut *
                (& mut *
                 __rtic_internal_local_resource_ts.get_mut()).as_mut_ptr(),
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
        const PRIORITY : u8 = 2u8 ; rtic :: export ::
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
                new(priority), EXTI : shared_resources ::
                EXTI_that_needs_to_be_locked :: new(priority),
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
                new(priority), EXTI : shared_resources ::
                EXTI_that_needs_to_be_locked :: new(priority),
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
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_handle_host_call_FQ : rtic ::
    RacyCell < rtic :: export :: SCFQ < 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ; #[link_section = ".uninit.rtic13"]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_handle_host_call_MyMono_INSTANTS :
    rtic :: RacyCell <
    [core :: mem :: MaybeUninit << DwtSystick < FREQ_CORE > as rtic ::
     Monotonic > :: Instant > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[link_section = ".uninit.rtic14"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_handle_host_call_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < [u8 ; 1024] > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_handle_dpi_FQ : rtic :: RacyCell <
    rtic :: export :: SCFQ < 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ; #[link_section = ".uninit.rtic15"]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_handle_dpi_MyMono_INSTANTS : rtic ::
    RacyCell <
    [core :: mem :: MaybeUninit << DwtSystick < FREQ_CORE > as rtic ::
     Monotonic > :: Instant > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[link_section = ".uninit.rtic16"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_handle_dpi_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < u16 > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ; impl < 'a >
    __rtic_internal_handle_dpiSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_handle_dpiSharedResources
            {
                mouse : shared_resources :: mouse_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)] #[doc(hidden)] pub enum P1_T
    { handle_dpi, handle_host_call, } #[doc(hidden)]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)] static
    __rtic_internal_P1_RQ : rtic :: RacyCell < rtic :: export :: SCRQ < P1_T,
    3 > > = rtic :: RacyCell :: new(rtic :: export :: Queue :: new()) ;
    #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch tasks at priority 1"] #[no_mangle]
    unsafe fn DMA1_STREAM0()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                while let Some((task, index)) =
                (& mut * __rtic_internal_P1_RQ.get_mut()).split().1.dequeue()
                {
                    match task
                    {
                        P1_T :: handle_dpi =>
                        {
                            let _0 =
                            (& *
                             __rtic_internal_handle_dpi_INPUTS.get()).get_unchecked(usize
                                                                                    ::
                                                                                    from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_handle_dpi_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            handle_dpi(handle_dpi :: Context :: new(priority),
                                       _0)
                        } P1_T :: handle_host_call =>
                        {
                            let _0 =
                            (& *
                             __rtic_internal_handle_host_call_INPUTS.get()).get_unchecked(usize
                                                                                          ::
                                                                                          from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_handle_host_call_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            handle_host_call(handle_host_call :: Context ::
                                             new(priority), _0)
                        }
                    }
                }
            }) ;
    } #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_TIMER_QUEUE_MARKER
    : rtic :: RacyCell < u32 > = rtic :: RacyCell :: new(0) ; #[doc(hidden)]
    #[allow(non_camel_case_types)] #[derive(Clone, Copy)] pub enum SCHED_T
    { handle_host_call, handle_dpi, } #[doc(hidden)]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)] static
    __rtic_internal_TQ_MyMono : rtic :: RacyCell < rtic :: export ::
    TimerQueue < DwtSystick < FREQ_CORE >, SCHED_T, 2 > > = rtic :: RacyCell
    ::
    new(rtic :: export ::
        TimerQueue(rtic :: export :: SortedLinkedList :: new_u16())) ;
    #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static
    __rtic_internal_MONOTONIC_STORAGE_MyMono : rtic :: RacyCell < Option <
    DwtSystick < FREQ_CORE > >> = rtic :: RacyCell :: new(None) ; #[no_mangle]
    #[allow(non_snake_case)] unsafe fn SysTick()
    {
        while let Some((task, index)) = rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) =
             (& mut *
              __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut()
             {
                 (& mut *
                  __rtic_internal_TQ_MyMono.get_mut()).dequeue(|| core :: mem
                                                               :: transmute ::
                                                               < _, rtic ::
                                                               export :: SYST
                                                               >
                                                               (()).disable_interrupt(),
                                                               mono)
             } else { core :: hint :: unreachable_unchecked() })
        {
            match task
            {
                SCHED_T :: handle_host_call =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ |
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).split().0.enqueue_unchecked((P1_T
                                                                                        ::
                                                                                        handle_host_call,
                                                                                        index)))
                    ; rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_STREAM0) ;
                } SCHED_T :: handle_dpi =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ |
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).split().0.enqueue_unchecked((P1_T
                                                                                        ::
                                                                                        handle_dpi,
                                                                                        index)))
                    ; rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_STREAM0) ;
                }
            }
        } rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) =
             (& mut *
              __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut()
             { mono.on_interrupt() ; }) ;
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_send :: < MouseKeyboardState > () ; rtic
            :: export :: assert_send :: < EXTI > () ; rtic :: export ::
            assert_send :: < UsbDevice < 'static, UsbBus < USB > > > () ; rtic
            :: export :: assert_send :: < HIDClass < 'static, UsbBus < USB > >
            > () ; rtic :: export :: assert_send :: < Button > () ; rtic ::
            export :: assert_send :: < u32 > () ; rtic :: export ::
            assert_send :: < [u8 ; 1024] > () ; rtic :: export :: assert_send
            :: < u16 > () ; rtic :: export :: assert_monotonic :: < DwtSystick
            < FREQ_CORE > > () ; rtic :: export :: interrupt :: disable() ;
            (0 ..
             1u8).for_each(| i |
                           (& mut *
                            __rtic_internal_handle_host_call_FQ.get_mut()).enqueue_unchecked(i))
            ;
            (0 ..
             1u8).for_each(| i |
                           (& mut *
                            __rtic_internal_handle_dpi_FQ.get_mut()).enqueue_unchecked(i))
            ; let mut core : rtic :: export :: Peripherals = rtic :: export ::
            Peripherals :: steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: DMA1_STREAM0 ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: DMA1_STREAM1 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: DMA1_STREAM0, rtic ::
                                   export ::
                                   logical2hw(1u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: DMA1_STREAM0) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 2u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI15_10, rtic :: export
                                   ::
                                   logical2hw(2u8, stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI15_10) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) - 2u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI0, rtic :: export ::
                                   logical2hw(2u8, stm32f4 :: stm32f401 ::
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
                   :: interrupt :: OTG_FS) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) -
              (1 << stm32f4 :: stm32f401 :: NVIC_PRIO_BITS) as usize)] ;
            core.SCB.set_priority(rtic :: export :: SystemHandler :: SysTick,
                                  rtic :: export ::
                                  logical2hw((1 << stm32f4 :: stm32f401 ::
                                              NVIC_PRIO_BITS), stm32f4 ::
                                             stm32f401 :: NVIC_PRIO_BITS),) ;
            if! < DwtSystick < FREQ_CORE > as rtic :: Monotonic > ::
            DISABLE_INTERRUPT_ON_EMPTY_QUEUE
            {
                core :: mem :: transmute :: < _, rtic :: export :: SYST >
                (()).enable_interrupt() ;
            } #[inline(never)] fn __rtic_init_resources < F > (f : F) where F
            : FnOnce() { f() ; }
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
                                      __rtic_internal_shared_resource_EXTI.get_mut().write(core
                                                                                           ::
                                                                                           mem
                                                                                           ::
                                                                                           MaybeUninit
                                                                                           ::
                                                                                           new(shared_resources.EXTI))
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
                                      ;
                                      __rtic_internal_local_resource_motion.get_mut().write(core
                                                                                            ::
                                                                                            mem
                                                                                            ::
                                                                                            MaybeUninit
                                                                                            ::
                                                                                            new(local_resources.motion))
                                      ;
                                      __rtic_internal_local_resource_ts.get_mut().write(core
                                                                                        ::
                                                                                        mem
                                                                                        ::
                                                                                        MaybeUninit
                                                                                        ::
                                                                                        new(local_resources.ts))
                                      ; monotonics.0.reset() ;
                                      __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut().write(Some(monotonics.0))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ;
            idle(idle :: Context ::
                 new(& rtic :: export :: Priority :: new(0)))
        }
    }
}