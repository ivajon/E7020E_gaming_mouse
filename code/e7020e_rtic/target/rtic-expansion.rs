#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f4 :: stm32f411 as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use super
    :: * ; #[doc = r" User code from within the module"]
    #[doc = r" User code end"] #[inline(always)] #[allow(non_snake_case)] fn
    init(_ : init :: Context) -> (Shared, Local, init :: Monotonics)
    { (Shared { }, Local { }, init :: Monotonics()) } #[allow(non_snake_case)]
    fn idle(_ : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; let rcc =
        unsafe { & mut * RCC :: get() } ; let gpioa = unsafe
        { & mut * GPIOA :: get() } ; let r = rcc.AHB1ENR.read() ;
        rcc.AHB1ENR.write(r | 1 << (0)) ; let r = gpioa.MODER.read() &!
        (0b11 << (5 * 2)) ; gpioa.MODER.write(r | 0b01 << (5 * 2)) ; loop
        {
            gpioa.BSRRH.write(1 << 5) ; wait(10_000) ;
            gpioa.BSRRL.write(1 << 5) ; wait(10_000) ;
        }
    } struct Shared { } struct Local { }
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
    #[doc = r" app module"] #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() ->!
        {
            rtic :: export :: interrupt :: disable() ; let mut core : rtic ::
            export :: Peripherals = rtic :: export :: Peripherals ::
            steal().into() ; #[inline(never)] fn __rtic_init_resources < F >
            (f : F) where F : FnOnce() { f() ; }
            __rtic_init_resources(||
                                  {
                                      let(shared_resources, local_resources,
                                          mut monotonics) =
                                      init(init :: Context ::
                                           new(core.into())) ; rtic :: export
                                      :: interrupt :: enable() ;
                                  }) ;
            idle(idle :: Context ::
                 new(& rtic :: export :: Priority :: new(0)))
        }
    }
}