#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use tracing::instrument;

fn main() {








    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello, world!\n"],
                                                         &match () {
                                                              () => [],
                                                          }));
    };
}
pub trait Foo where Self: std::marker::Send {
    fn foo<'life0, 'async_trait>(&'life0 self)
    ->
        ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> +
                             ::core::marker::Send + 'async_trait>>
    where
    'life0: 'async_trait,
    Self: 'async_trait;
}
struct FooImpl;
impl Foo for FooImpl {
    fn foo<'life0, 'async_trait>(&'life0 self)
     ->
         ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> +
                              ::core::marker::Send + 'async_trait>> where
     'life0: 'async_trait, Self: 'async_trait {
        let __tracing_attr_span =
            {
                use ::tracing::callsite;
                use ::tracing::callsite::Callsite;
                let callsite =
                    {
                        use ::tracing::{callsite, subscriber::Interest,
                                        Metadata, __macro_support::*};
                        struct MyCallsite;
                        static META: Metadata<'static> =
                            {
                                ::tracing_core::metadata::Metadata::new("foo",
                                                                        "bug_repro",
                                                                        tracing::Level::INFO,
                                                                        Some("src/main.rs"),
                                                                        Some(21u32),
                                                                        Some("bug_repro"),
                                                                        ::tracing_core::field::FieldSet::new(&[],
                                                                                                             ::tracing_core::callsite::Identifier(&MyCallsite)),
                                                                        ::tracing::metadata::Kind::SPAN)
                            };
                        static INTEREST: AtomicUsize = AtomicUsize::new(0);
                        static REGISTRATION: Once = Once::new();
                        impl MyCallsite {
                            #[inline]
                            fn interest(&self) -> Interest {
                                match INTEREST.load(Ordering::Relaxed) {
                                    0 => Interest::never(),
                                    2 => Interest::always(),
                                    _ => Interest::sometimes(),
                                }
                            }
                        }
                        impl callsite::Callsite for MyCallsite {
                            fn set_interest(&self, interest: Interest) {
                                let interest =
                                    match () {
                                        _ if interest.is_never() => 0,
                                        _ if interest.is_always() => 2,
                                        _ => 1,
                                    };
                                INTEREST.store(interest, Ordering::SeqCst);
                            }
                            fn metadata(&self) -> &Metadata { &META }
                        }
                        REGISTRATION.call_once(||
                                                   {
                                                       callsite::register(&MyCallsite);
                                                   });
                        &MyCallsite
                    };
                let meta = callsite.metadata();
                if ::tracing::dispatcher::has_been_set() &&
                       tracing::Level::INFO <=
                           ::tracing::level_filters::STATIC_MAX_LEVEL &&
                       {
                           let interest = callsite.interest();
                           if interest.is_never() {
                               false
                           } else if interest.is_always() {
                               true
                           } else {
                               let meta = callsite.metadata();
                               ::tracing::dispatcher::get_default(|current|
                                                                      current.enabled(meta))
                           }
                       } {
                    ::tracing::Span::new(meta,
                                         &{ meta.fields().value_set(&[]) })
                } else {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            let span = ::tracing::Span::new_disabled(meta);
                            span.record_all(&{
                                                 meta.fields().value_set(&[])
                                             });
                            span
                        }
                    } else { { ::tracing::Span::none() } }
                }
            };
        let __tracing_attr_guard = __tracing_attr_span.enter();
        {
            #[allow(clippy :: used_underscore_binding)]
            async fn __foo(_self: &FooImpl) { }
            Box::pin(__foo::<>(self))
        }
    }
}
async fn top_level() {
    let __tracing_attr_span =
        {
            use ::tracing::callsite;
            use ::tracing::callsite::Callsite;
            let callsite =
                {
                    use ::tracing::{callsite, subscriber::Interest, Metadata,
                                    __macro_support::*};
                    struct MyCallsite;
                    static META: Metadata<'static> =
                        {
                            ::tracing_core::metadata::Metadata::new("top_level",
                                                                    "bug_repro",
                                                                    tracing::Level::INFO,
                                                                    Some("src/main.rs"),
                                                                    Some(27u32),
                                                                    Some("bug_repro"),
                                                                    ::tracing_core::field::FieldSet::new(&[],
                                                                                                         ::tracing_core::callsite::Identifier(&MyCallsite)),
                                                                    ::tracing::metadata::Kind::SPAN)
                        };
                    static INTEREST: AtomicUsize = AtomicUsize::new(0);
                    static REGISTRATION: Once = Once::new();
                    impl MyCallsite {
                        #[inline]
                        fn interest(&self) -> Interest {
                            match INTEREST.load(Ordering::Relaxed) {
                                0 => Interest::never(),
                                2 => Interest::always(),
                                _ => Interest::sometimes(),
                            }
                        }
                    }
                    impl callsite::Callsite for MyCallsite {
                        fn set_interest(&self, interest: Interest) {
                            let interest =
                                match () {
                                    _ if interest.is_never() => 0,
                                    _ if interest.is_always() => 2,
                                    _ => 1,
                                };
                            INTEREST.store(interest, Ordering::SeqCst);
                        }
                        fn metadata(&self) -> &Metadata { &META }
                    }
                    REGISTRATION.call_once(||
                                               {
                                                   callsite::register(&MyCallsite);
                                               });
                    &MyCallsite
                };
            let meta = callsite.metadata();
            if ::tracing::dispatcher::has_been_set() &&
                   tracing::Level::INFO <=
                       ::tracing::level_filters::STATIC_MAX_LEVEL &&
                   {
                       let interest = callsite.interest();
                       if interest.is_never() {
                           false
                       } else if interest.is_always() {
                           true
                       } else {
                           let meta = callsite.metadata();
                           ::tracing::dispatcher::get_default(|current|
                                                                  current.enabled(meta))
                       }
                   } {
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        let span = ::tracing::Span::new_disabled(meta);
                        span.record_all(&{ meta.fields().value_set(&[]) });
                        span
                    }
                } else { { ::tracing::Span::none() } }
            }
        };
    tracing_futures::Instrument::instrument(async move  { { } },
                                            __tracing_attr_span).await
}
