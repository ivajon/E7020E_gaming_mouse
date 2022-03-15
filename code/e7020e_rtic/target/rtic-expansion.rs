#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal :: gpio ::
    * ; use stm32f4xx_hal ::
    { prelude :: *, serial :: { config :: Config, Event, Rx, Serial, Tx }, } ;
    use nb :: block ; use stm32f4 :: stm32f411 :: USART2 ;
    #[doc = r" User code from within the module"] #[doc = r" User code end"]
    #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let device = cx.device ; let
        pa = device.GPIOA.split() ; let tx_pin = pa.pa2.into_alternate :: < 7
        > () ; let rx_pin = pa.pa3.into_alternate :: < 7 > () ; let rcc =
        device.RCC.constrain() ; let clocks = rcc.cfgr.freeze() ; let mut
        serial : Serial < USART2, _ > = Serial ::
        new(device.USART2, (tx_pin, rx_pin), Config ::
            default().baudrate(115_200.bps()), & clocks,).unwrap() ;
        serial.listen(Event :: Rxne) ; let(tx, rx) = serial.split() ;
        (Shared { tx, rx }, Local { }, init :: Monotonics())
    } #[allow(non_snake_case)] fn idle(_cx : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; rprintln!
        ("idle") ; loop { }
    } #[allow(non_snake_case)] fn usart2(cx : usart2 :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let rx =
        cx.shared.rx ; let data = rx.read().unwrap() ; worker ::
        spawn(data).unwrap() ;
    } #[allow(non_snake_case)] fn worker(cx : worker :: Context, data : u8)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let tx =
        cx.shared.tx ; tx.write(data).unwrap() ; rprintln! ("data {}", data) ;
    } struct Shared { tx : Tx < USART2 >, rx : Rx < USART2 >, } struct Local
    { } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
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
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context < >
    { } impl < > __rtic_internal_idle_Context < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        { __rtic_internal_idle_Context { } }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    { pub use super :: __rtic_internal_idle_Context as Context ; }
    #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `usart2` has access to"] pub struct
    __rtic_internal_usart2SharedResources < 'a >
    { pub rx : & 'a mut Rx < USART2 >, } #[doc = r" Execution context"]
    #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_usart2_Context < 'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        usart2 :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_usart2_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usart2_Context
            { shared : usart2 :: SharedResources :: new(priority), }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod usart2
    {
        #[doc(inline)] pub use super :: __rtic_internal_usart2SharedResources
        as SharedResources ; pub use super :: __rtic_internal_usart2_Context
        as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `worker` has access to"] pub struct
    __rtic_internal_workerSharedResources < 'a >
    { pub tx : & 'a mut Tx < USART2 >, } #[doc = r" Execution context"]
    #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_worker_Context < 'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        worker :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_worker_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_worker_Context
            { shared : worker :: SharedResources :: new(priority), }
        }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_worker_spawn(_0 : u8,) -> Result < (), u8 >
    {
        let input = _0 ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_worker_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_worker_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                            ::
                                                                            from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              worker,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f411 :: interrupt :: EXTI0) ; Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod worker
    {
        #[doc(inline)] pub use super :: __rtic_internal_workerSharedResources
        as SharedResources ; pub use super :: __rtic_internal_worker_Context
        as Context ; pub use super :: __rtic_internal_worker_spawn as spawn ;
    } #[doc = r" app module"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic13218"] static
    __rtic_internal_shared_resource_tx : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Tx < USART2 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic13219"] static
    __rtic_internal_shared_resource_rx : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Rx < USART2 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; #[allow(non_snake_case)]
    #[no_mangle] unsafe fn USART2()
    {
        const PRIORITY : u8 = 2u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                usart2(usart2 :: Context ::
                       new(& rtic :: export :: Priority :: new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_usart2SharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_usart2SharedResources
            {
                rx : & mut *
                (& mut *
                 __rtic_internal_shared_resource_rx.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_worker_FQ : rtic :: RacyCell < rtic
    :: export :: SCFQ < 129 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ;
    #[link_section = ".uninit.rtic13220"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_worker_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < u8 > ; 128] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
         MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core
         :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
         uninit(), core :: mem :: MaybeUninit :: uninit(),]) ; impl < 'a >
    __rtic_internal_workerSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_workerSharedResources
            {
                tx : & mut *
                (& mut *
                 __rtic_internal_shared_resource_tx.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)] #[doc(hidden)] pub enum P1_T { worker, }
    #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_P1_RQ : rtic ::
    RacyCell < rtic :: export :: SCRQ < P1_T, 129 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ; #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch tasks at priority 1"] #[no_mangle]
    unsafe fn EXTI0()
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
                        P1_T :: worker =>
                        {
                            let _0 =
                            (& *
                             __rtic_internal_worker_INPUTS.get()).get_unchecked(usize
                                                                                ::
                                                                                from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_worker_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            worker(worker :: Context :: new(priority), _0)
                        }
                    }
                }
            }) ;
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_send :: < Tx < USART2 > > () ; rtic ::
            export :: assert_send :: < Rx < USART2 > > () ; rtic :: export ::
            assert_send :: < u8 > () ; rtic :: export :: interrupt ::
            disable() ;
            (0 ..
             128u8).for_each(| i |
                             (& mut *
                              __rtic_internal_worker_FQ.get_mut()).enqueue_unchecked(i))
            ; let mut core : rtic :: export :: Peripherals = rtic :: export ::
            Peripherals :: steal().into() ; let _ =
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
            interrupt :: EXTI0 ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: EXTI0, rtic :: export ::
                                   logical2hw(1u8, stm32f4 :: stm32f411 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI0) ; let _ =
            [() ;
             ((1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) - 2u8 as usize)] ;
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                                   :: interrupt :: USART2, rtic :: export ::
                                   logical2hw(2u8, stm32f4 :: stm32f411 ::
                                              NVIC_PRIO_BITS),) ; rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: USART2) ; #[inline(never)] fn
            __rtic_init_resources < F > (f : F) where F : FnOnce() { f() ; }
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