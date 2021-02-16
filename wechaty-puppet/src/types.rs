use futures::future::{BoxFuture, Future};

pub struct AsyncFnPtr<T, R> {
    func: Box<dyn Fn(T) -> BoxFuture<'static, R> + Send + 'static>,
}

impl<T, R> AsyncFnPtr<T, R>
where
    T: 'static,
{
    fn new<Fut, F>(f: F) -> AsyncFnPtr<T, Fut::Output>
    where
        F: Fn(T) -> Fut + Send + 'static,
        Fut: Future<Output = R> + Send + 'static,
    {
        AsyncFnPtr {
            func: Box::new(move |t: T| Box::pin(f(t))),
        }
    }
    pub async fn run(&self, t: T) -> R {
        (self.func)(t).await
    }
}

pub trait IntoAsyncFnPtr<T, R>
where
    T: 'static,
{
    fn into(self) -> AsyncFnPtr<T, R>;
}

impl<F, T, R, Fut> IntoAsyncFnPtr<T, R> for F
where
    F: Fn(T) -> Fut + Send + 'static,
    T: 'static,
    Fut: Future<Output = R> + Send + 'static,
{
    fn into(self) -> AsyncFnPtr<T, R> {
        AsyncFnPtr::new(self)
    }
}
