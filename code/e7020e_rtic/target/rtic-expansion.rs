#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
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
    } use cortex_m_semihosting :: hprintln ; use dwt_systick_monotonic :: * ;
    use stm32f4xx_hal :: gpio :: * ;
    #[doc = r" User code from within the module"] type LED = Pin < Output <
    PushPull >, 'A', 5 > ; const FREQ_CORE : u32 = 16_000_000 ; type MyMono =
    DwtSystick < FREQ_CORE > ; #[doc = r" User code end"] #[inline(always)]
    #[allow(non_snake_case)] fn init(cx : init :: Context) ->
    (Shared, Local, init :: Monotonics)
    {
        hprintln! ("init").ok() ; let systick = cx.core.SYST ; let mut dcb =
        cx.core.DCB ; let dwt = cx.core.DWT ; let mono = DwtSystick ::
        new(& mut dcb, dwt, systick, FREQ_CORE) ; let gpioa = cx.device.GPIOA
        ; let pa = gpioa.split() ; let led = pa.pa5.into_push_pull_output() ;
        led_on :: spawn().ok() ;
        (Shared { led }, Local {}, init :: Monotonics(mono))
    } #[allow(non_snake_case)] fn idle(_cx : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; hprintln!
        ("idle").ok() ; loop {}
    } #[allow(non_snake_case)] fn led_on(cx : led_on :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; hprintln!
        ("led_on").ok() ; cx.shared.led.set_high() ; led_off ::
        spawn_after(1.secs()).ok() ;
    } #[allow(non_snake_case)] fn led_off(cx : led_off :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; hprintln!
        ("led_off").ok() ; cx.shared.led.set_low() ; led_on ::
        spawn_after(1.secs()).ok() ;
    } struct Shared { led : LED, } struct Local {}
    #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_Monotonics(pub DwtSystick < FREQ_CORE >) ;
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
    {} impl < > __rtic_internal_idle_Context < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        { __rtic_internal_idle_Context {} }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    { pub use super :: __rtic_internal_idle_Context as Context ; }
    #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `led_on` has access to"] pub struct
    __rtic_internal_led_onSharedResources < 'a > { pub led : & 'a mut LED, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_led_on_Context <
    'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        led_on :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_led_on_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_led_on_Context
            { shared : led_on :: SharedResources :: new(priority), }
        }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_led_on_spawn() -> Result < (), () >
    {
        let input = () ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_led_on_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_led_on_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                            ::
                                                                            from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              led_on,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f411 :: interrupt :: EXTI0) ; Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_led_on_MyMono_SpawnHandle { #[doc(hidden)] marker : u32, }
    impl core :: fmt :: Debug for __rtic_internal_led_on_MyMono_SpawnHandle
    {
        fn fmt(& self, f : & mut core :: fmt :: Formatter < '_ >) -> core ::
        fmt :: Result { f.debug_struct("MyMono::SpawnHandle").finish() }
    } impl __rtic_internal_led_on_MyMono_SpawnHandle
    {
        pub fn cancel(self) -> Result < (), () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let tq = & mut * __rtic_internal_TQ_MyMono.get_mut() ; if
                     let Some((_task, index)) = tq.cancel_marker(self.marker)
                     {
                         let msg =
                         (& *
                          __rtic_internal_led_on_INPUTS.get()).get_unchecked(usize
                                                                             ::
                                                                             from(index)).as_ptr().read()
                         ;
                         (& mut *
                          __rtic_internal_led_on_FQ.get_mut()).split().0.enqueue_unchecked(index)
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
                                      set_pendst()).map(| _ | led_on :: MyMono
                                                        :: SpawnHandle
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
    __rtic_internal_led_on_MyMono_spawn_after(duration : < MyMono as rtic ::
                                              Monotonic > :: Duration) ->
    Result < led_on :: MyMono :: SpawnHandle, () >
    {
        let instant = monotonics :: MyMono :: now() ;
        __rtic_internal_led_on_MyMono_spawn_at(instant + duration)
    } #[doc = r" Spawns the task at a fixed time instant"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_led_on_MyMono_spawn_at(instant : < MyMono as rtic ::
                                           Monotonic > :: Instant) -> Result <
    led_on :: MyMono :: SpawnHandle, () >
    {
        unsafe
        {
            let input = () ; if let Some(index) = rtic :: export :: interrupt
            ::
            free(| _ |
                 (& mut * __rtic_internal_led_on_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_led_on_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                            ::
                                                                            from(index)).as_mut_ptr().write(input)
                ;
                (& mut *
                 __rtic_internal_led_on_MyMono_INSTANTS.get_mut()).get_unchecked_mut(usize
                                                                                     ::
                                                                                     from(index)).as_mut_ptr().write(instant)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         let marker =
                         __rtic_internal_TIMER_QUEUE_MARKER.get().read() ; let
                         nr = rtic :: export :: NotReady
                         { instant, index, task : SCHED_T :: led_on, marker, }
                         ;
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
                         ; Ok(led_on :: MyMono :: SpawnHandle { marker })
                     })
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod led_on
    {
        #[doc(inline)] pub use super :: __rtic_internal_led_onSharedResources
        as SharedResources ; pub use super :: __rtic_internal_led_on_Context
        as Context ; pub use super :: __rtic_internal_led_on_spawn as spawn ;
        pub use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ; pub mod MyMono
        {
            pub use super :: super ::
            __rtic_internal_led_on_MyMono_spawn_after as spawn_after ; pub use
            super :: super :: __rtic_internal_led_on_MyMono_spawn_at as
            spawn_at ; pub use super :: super ::
            __rtic_internal_led_on_MyMono_SpawnHandle as SpawnHandle ;
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `led_off` has access to"] pub struct
    __rtic_internal_led_offSharedResources < 'a > { pub led : & 'a mut LED, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_led_off_Context
    < 'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        led_off :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_led_off_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_led_off_Context
            { shared : led_off :: SharedResources :: new(priority), }
        }
    } #[doc = r" Spawns the task directly"] pub fn
    __rtic_internal_led_off_spawn() -> Result < (), () >
    {
        let input = () ; unsafe
        {
            if let Some(index) = rtic :: export :: interrupt ::
            free(| _ |
                 (& mut * __rtic_internal_led_off_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_led_off_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                             ::
                                                                             from(index)).as_mut_ptr().write(input)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).enqueue_unchecked((P1_T
                                                                              ::
                                                                              led_off,
                                                                              index))
                         ;
                     }) ; rtic ::
                pend(stm32f4 :: stm32f411 :: interrupt :: EXTI0) ; Ok(())
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_led_off_MyMono_SpawnHandle
    { #[doc(hidden)] marker : u32, } impl core :: fmt :: Debug for
    __rtic_internal_led_off_MyMono_SpawnHandle
    {
        fn fmt(& self, f : & mut core :: fmt :: Formatter < '_ >) -> core ::
        fmt :: Result { f.debug_struct("MyMono::SpawnHandle").finish() }
    } impl __rtic_internal_led_off_MyMono_SpawnHandle
    {
        pub fn cancel(self) -> Result < (), () >
        {
            rtic :: export :: interrupt ::
            free(| _ | unsafe
                 {
                     let tq = & mut * __rtic_internal_TQ_MyMono.get_mut() ; if
                     let Some((_task, index)) = tq.cancel_marker(self.marker)
                     {
                         let msg =
                         (& *
                          __rtic_internal_led_off_INPUTS.get()).get_unchecked(usize
                                                                              ::
                                                                              from(index)).as_ptr().read()
                         ;
                         (& mut *
                          __rtic_internal_led_off_FQ.get_mut()).split().0.enqueue_unchecked(index)
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
                                      set_pendst()).map(| _ | led_off ::
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
    __rtic_internal_led_off_MyMono_spawn_after(duration : < MyMono as rtic ::
                                               Monotonic > :: Duration) ->
    Result < led_off :: MyMono :: SpawnHandle, () >
    {
        let instant = monotonics :: MyMono :: now() ;
        __rtic_internal_led_off_MyMono_spawn_at(instant + duration)
    } #[doc = r" Spawns the task at a fixed time instant"]
    #[allow(non_snake_case)] pub fn
    __rtic_internal_led_off_MyMono_spawn_at(instant : < MyMono as rtic ::
                                            Monotonic > :: Instant) -> Result
    < led_off :: MyMono :: SpawnHandle, () >
    {
        unsafe
        {
            let input = () ; if let Some(index) = rtic :: export :: interrupt
            ::
            free(| _ |
                 (& mut * __rtic_internal_led_off_FQ.get_mut()).dequeue())
            {
                (& mut *
                 __rtic_internal_led_off_INPUTS.get_mut()).get_unchecked_mut(usize
                                                                             ::
                                                                             from(index)).as_mut_ptr().write(input)
                ;
                (& mut *
                 __rtic_internal_led_off_MyMono_INSTANTS.get_mut()).get_unchecked_mut(usize
                                                                                      ::
                                                                                      from(index)).as_mut_ptr().write(instant)
                ; rtic :: export :: interrupt ::
                free(| _ |
                     {
                         let marker =
                         __rtic_internal_TIMER_QUEUE_MARKER.get().read() ; let
                         nr = rtic :: export :: NotReady
                         {
                             instant, index, task : SCHED_T :: led_off,
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
                         ; Ok(led_off :: MyMono :: SpawnHandle { marker })
                     })
            } else { Err(input) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod led_off
    {
        #[doc(inline)] pub use super :: __rtic_internal_led_offSharedResources
        as SharedResources ; pub use super :: __rtic_internal_led_off_Context
        as Context ; pub use super :: __rtic_internal_led_off_spawn as spawn ;
        pub use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ; pub mod MyMono
        {
            pub use super :: super ::
            __rtic_internal_led_off_MyMono_spawn_after as spawn_after ; pub
            use super :: super :: __rtic_internal_led_off_MyMono_spawn_at as
            spawn_at ; pub use super :: super ::
            __rtic_internal_led_off_MyMono_SpawnHandle as SpawnHandle ;
        }
    } #[doc = r" app module"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic754"] static
    __rtic_internal_shared_resource_led : rtic :: RacyCell < core :: mem ::
    MaybeUninit < LED >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_led_on_FQ : rtic :: RacyCell < rtic
    :: export :: SCFQ < 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ;
    #[link_section = ".uninit.rtic755"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_led_on_MyMono_INSTANTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit << DwtSystick < FREQ_CORE > as rtic ::
     Monotonic > :: Instant > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[link_section = ".uninit.rtic756"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_led_on_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < () > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ; impl < 'a >
    __rtic_internal_led_onSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_led_onSharedResources
            {
                led : & mut *
                (& mut *
                 __rtic_internal_shared_resource_led.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_led_off_FQ : rtic :: RacyCell < rtic
    :: export :: SCFQ < 2 > > = rtic :: RacyCell ::
    new(rtic :: export :: Queue :: new()) ;
    #[link_section = ".uninit.rtic757"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_led_off_MyMono_INSTANTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit << DwtSystick < FREQ_CORE > as rtic ::
     Monotonic > :: Instant > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ;
    #[link_section = ".uninit.rtic758"] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] #[doc(hidden)] static
    __rtic_internal_led_off_INPUTS : rtic :: RacyCell <
    [core :: mem :: MaybeUninit < () > ; 1] > = rtic :: RacyCell ::
    new([core :: mem :: MaybeUninit :: uninit(),]) ; impl < 'a >
    __rtic_internal_led_offSharedResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_led_offSharedResources
            {
                led : & mut *
                (& mut *
                 __rtic_internal_shared_resource_led.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)] #[doc(hidden)] pub enum P1_T { led_off, led_on, }
    #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_P1_RQ : rtic ::
    RacyCell < rtic :: export :: SCRQ < P1_T, 3 > > = rtic :: RacyCell ::
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
                        P1_T :: led_off =>
                        {
                            let() =
                            (& *
                             __rtic_internal_led_off_INPUTS.get()).get_unchecked(usize
                                                                                 ::
                                                                                 from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_led_off_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            led_off(led_off :: Context :: new(priority))
                        } P1_T :: led_on =>
                        {
                            let() =
                            (& *
                             __rtic_internal_led_on_INPUTS.get()).get_unchecked(usize
                                                                                ::
                                                                                from(index)).as_ptr().read()
                            ;
                            (& mut *
                             __rtic_internal_led_on_FQ.get_mut()).split().0.enqueue_unchecked(index)
                            ; let priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ;
                            led_on(led_on :: Context :: new(priority))
                        }
                    }
                }
            }) ;
    } #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_TIMER_QUEUE_MARKER
    : rtic :: RacyCell < u32 > = rtic :: RacyCell :: new(0) ; #[doc(hidden)]
    #[allow(non_camel_case_types)] #[derive(Clone, Copy)] pub enum SCHED_T
    { led_on, led_off, } #[doc(hidden)] #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)] static __rtic_internal_TQ_MyMono : rtic
    :: RacyCell < rtic :: export :: TimerQueue < DwtSystick < FREQ_CORE >,
    SCHED_T, 2 > > = rtic :: RacyCell ::
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
                SCHED_T :: led_on =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ |
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).split().0.enqueue_unchecked((P1_T
                                                                                        ::
                                                                                        led_on,
                                                                                        index)))
                    ; rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: EXTI0) ;
                } SCHED_T :: led_off =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ |
                         (& mut *
                          __rtic_internal_P1_RQ.get_mut()).split().0.enqueue_unchecked((P1_T
                                                                                        ::
                                                                                        led_off,
                                                                                        index)))
                    ; rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: EXTI0) ;
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
            rtic :: export :: assert_send :: < LED > () ; rtic :: export ::
            assert_monotonic :: < DwtSystick < FREQ_CORE > > () ; rtic ::
            export :: interrupt :: disable() ;
            (0 ..
             1u8).for_each(| i |
                           (& mut *
                            __rtic_internal_led_on_FQ.get_mut()).enqueue_unchecked(i))
            ;
            (0 ..
             1u8).for_each(| i |
                           (& mut *
                            __rtic_internal_led_off_FQ.get_mut()).enqueue_unchecked(i))
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
             ((1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) -
              (1 << stm32f4 :: stm32f411 :: NVIC_PRIO_BITS) as usize)] ;
            core.SCB.set_priority(rtic :: export :: SystemHandler :: SysTick,
                                  rtic :: export ::
                                  logical2hw((1 << stm32f4 :: stm32f411 ::
                                              NVIC_PRIO_BITS), stm32f4 ::
                                             stm32f411 :: NVIC_PRIO_BITS),) ;
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
                                      __rtic_internal_shared_resource_led.get_mut().write(core
                                                                                          ::
                                                                                          mem
                                                                                          ::
                                                                                          MaybeUninit
                                                                                          ::
                                                                                          new(shared_resources.led))
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