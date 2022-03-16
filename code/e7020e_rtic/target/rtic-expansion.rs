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
    } use app :: pmw3389 :: Pmw3389 ; use dwt_systick_monotonic :: * ; use
    embedded_hal :: spi :: MODE_3 ; use rtt_target ::
    { rprintln, rtt_init_print } ; use stm32f4xx_hal ::
    {
        gpio :: { Alternate, Output, Pin, PushPull, Speed }, prelude :: *, spi
        :: { Spi, TransferModeNormal }, timer :: Delay,
    } ; use stm32f4 :: stm32f401 :: { SPI1, TIM5 } ;
    #[doc = r" User code from within the module"] type SCK = Pin < Alternate <
    PushPull, 5_u8 >, 'A', 5_u8 > ; type MOSI = Pin < Alternate < PushPull,
    5_u8 >, 'A', 7_u8 > ; type MISO = Pin < Alternate < PushPull, 5_u8 >, 'A',
    6_u8 > ; type CS = Pin < Output < PushPull >, 'A', 4_u8 > ; type SPI = Spi
    < SPI1, (SCK, MISO, MOSI), TransferModeNormal > ; type DELAY = Delay <
    TIM5, 1000000_u32 > ; type PMW3389 = Pmw3389 < SPI, CS, DELAY > ; const
    FREQ_CORE : u32 = 48_000_000 ; type MyMono = DwtSystick < FREQ_CORE > ;
    #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(cx : init :: Context) -> (Shared, Local, init :: Monotonics)
    {
        rtt_init_print! () ; rprintln! ("init") ; let systick = cx.core.SYST ;
        let mut dcb = cx.core.DCB ; let dwt = cx.core.DWT ; let device =
        cx.device ; let rcc = device.RCC ; let clocks =
        rcc.constrain().cfgr.sysclk(48.MHz()).freeze() ; let _mono =
        DwtSystick :: new(& mut dcb, dwt, systick, clocks.sysclk().raw()) ;
        let delay : DELAY = device.TIM5.delay_us(& clocks) ; let gpioa =
        device.GPIOA.split() ; let gpioc = device.GPIOC.split() ; let sck :
        SCK = gpioa.pa5.into_alternate().set_speed(Speed :: VeryHigh) ; let
        miso : MISO = gpioa.pa6.into_alternate().set_speed(Speed :: High) ;
        let mosi : MOSI = gpioa.pa7.into_alternate().set_speed(Speed :: High)
        ; let cs : CS =
        gpioa.pa4.into_push_pull_output().set_speed(Speed :: High) ; let spi :
        SPI = Spi ::
        new(device.SPI1, (sck, miso, mosi), MODE_3, 1.MHz(), & clocks) ; let
        pmw3389 : PMW3389 = Pmw3389 :: new(spi, cs, delay).unwrap() ;
        (Shared {}, Local { pmw3389 }, init :: Monotonics(_mono))
    } #[allow(non_snake_case)] fn idle(cx : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let pmw3389
        = cx.local.pmw3389 ; pmw3389.set_cpi(8000).unwrap() ; let mut x_acc :
        i32 = 0 ; loop
        {
            let status = pmw3389.read_status().unwrap() ; x_acc += status.dx
            as i32 ; rprintln! ("acc {} dx, dy = {:?}", x_acc, status) ;
            pmw3389.delay.delay_ms(100_u32) ;
        }
    } struct Shared {} struct Local { pmw3389 : PMW3389, }
    #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
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
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                device : stm32f4 :: stm32f401 :: Peripherals :: steal(), cs :
                rtic :: export :: CriticalSection :: new(), core,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        pub use super :: __rtic_internal_Monotonics as Monotonics ; pub use
        super :: __rtic_internal_init_Context as Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `idle` has access to"] pub struct
    __rtic_internal_idleLocalResources < >
    { pub pmw3389 : & 'static mut PMW3389, } #[doc = r" Execution context"]
    #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_idle_Context < >
    {
        #[doc = r" Local Resources this task has access to"] pub local : idle
        :: LocalResources < >,
    } impl < > __rtic_internal_idle_Context < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idle_Context
            { local : idle :: LocalResources :: new(), }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleLocalResources as
        LocalResources ; pub use super :: __rtic_internal_idle_Context as
        Context ;
    } #[doc = r" app module"] impl < > __rtic_internal_idleLocalResources < >
    {
        #[inline(always)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idleLocalResources
            {
                pmw3389 : & mut *
                (& mut *
                 __rtic_internal_local_resource_pmw3389.get_mut()).as_mut_ptr(),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1572"] static
    __rtic_internal_local_resource_pmw3389 : rtic :: RacyCell < core :: mem ::
    MaybeUninit < PMW3389 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; #[doc(hidden)]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)] static
    __rtic_internal_TIMER_QUEUE_MARKER : rtic :: RacyCell < u32 > = rtic ::
    RacyCell :: new(0) ; #[doc(hidden)] #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)] pub enum SCHED_T {} #[doc(hidden)]
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)] static
    __rtic_internal_TQ_MyMono : rtic :: RacyCell < rtic :: export ::
    TimerQueue < DwtSystick < FREQ_CORE >, SCHED_T, 0 > > = rtic :: RacyCell
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
        { match task {} } rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) =
             (& mut *
              __rtic_internal_MONOTONIC_STORAGE_MyMono.get_mut()).as_mut()
             { mono.on_interrupt() ; }) ;
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: assert_monotonic :: < DwtSystick < FREQ_CORE > >
            () ; rtic :: export :: interrupt :: disable() ; let mut core :
            rtic :: export :: Peripherals = rtic :: export :: Peripherals ::
            steal().into() ; let _ =
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
                                      __rtic_internal_local_resource_pmw3389.get_mut().write(core
                                                                                             ::
                                                                                             mem
                                                                                             ::
                                                                                             MaybeUninit
                                                                                             ::
                                                                                             new(local_resources.pmw3389))
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