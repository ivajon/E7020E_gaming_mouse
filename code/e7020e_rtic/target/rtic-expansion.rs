#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use nb ::
    block ; use rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal
    :: gpio :: * ; use stm32f4xx_hal ::
    { prelude :: *, serial :: { config :: Config, Rx, Serial, Tx }, } ; use
    stm32f4 :: stm32f411 :: USART2 ;
    #[doc = r" User code from within the module"] #[doc = r" User code end"]
    #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let device = cx.device ; let
        pa = device.GPIOA.split() ; let tx_pin = pa.pa2.into_alternate :: < 7
        > () ; let rx_pin = pa.pa3.into_alternate :: < 7 > () ; let rcc =
        device.RCC.constrain() ; let clocks = rcc.cfgr.freeze() ; let serial :
        Serial < USART2, _ > = Serial ::
        new(device.USART2, (tx_pin, rx_pin), Config ::
            default().baudrate(115_200.bps()), & clocks,).unwrap() ;
        let(tx, rx) = serial.split() ;
        (Shared { tx, rx }, Local {}, init :: Monotonics())
    } #[allow(non_snake_case)] fn idle(cx : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; rprintln!
        ("idle") ; let rx = cx.shared.rx ; let tx = cx.shared.tx ; loop
        {
            match block! (rx.read())
            {
                Ok(byte) =>
                { rprintln! ("Ok {:?}", byte) ; tx.write(byte).unwrap() ; }
                Err(err) => { rprintln! ("Error {:?}", err) ; }
            }
        }
    } struct Shared { tx : Tx < USART2 >, rx : Rx < USART2 >, } struct Local
    {} #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_Monotonics() ;
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals"] pub device : stm32f4 ::
        stm32f411 :: Peripherals, #[doc = r" Critical section token for init"]
        pub cs : rtic :: export :: CriticalSection < 'a >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                device : stm32f4 :: stm32f411 :: Peripherals :: steal(), cs :
                rtic :: export :: CriticalSection :: new(), core,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        pub use super :: __rtic_internal_Monotonics as Monotonics ; pub use
        super :: __rtic_internal_init_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `idle` has access to"] pub struct
    __rtic_internal_idleSharedResources < >
    {
        pub tx : & 'static mut Tx < USART2 >, pub rx : & 'static mut Rx <
        USART2 >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context < >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        idle :: SharedResources < >,
    } impl < > __rtic_internal_idle_Context < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idle_Context
            { shared : idle :: SharedResources :: new(priority), }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleSharedResources as
        SharedResources ; pub use super :: __rtic_internal_idle_Context as
        Context ;
    } #[doc = r" app module"] impl < > __rtic_internal_idleSharedResources < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idleSharedResources
            {
                tx : & mut *
                (& mut *
                 __rtic_internal_shared_resource_tx.get_mut()).as_mut_ptr(),
                rx : & mut *
                (& mut *
                 __rtic_internal_shared_resource_rx.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic50"] static
    __rtic_internal_shared_resource_tx : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Tx < USART2 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic51"] static
    __rtic_internal_shared_resource_rx : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Rx < USART2 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; #[doc(hidden)] mod rtic_ext
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
                                           new(core.into())) ;
                                      __rtic_internal_shared_resource_tx.get_mut().write(core
                                                                                         ::
                                                                                         mem
                                                                                         ::
                                                                                         MaybeUninit
                                                                                         ::
                                                                                         new(shared_resources.tx))
                                      ;
                                      __rtic_internal_shared_resource_rx.get_mut().write(core
                                                                                         ::
                                                                                         mem
                                                                                         ::
                                                                                         MaybeUninit
                                                                                         ::
                                                                                         new(shared_resources.rx))
                                      ; rtic :: export :: interrupt ::
                                      enable() ;
                                  }) ;
            idle(idle :: Context ::
                 new(& rtic :: export :: Priority :: new(0)))
        }
    }
}