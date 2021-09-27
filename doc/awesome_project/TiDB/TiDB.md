# TiDB

### Arch
```
    Service - Storage - TiKV(存储引擎层)  
                              / | \
                              
```



### TiKV
KV 操作分为 RawKV 和 TxnKV
RawKV包含raw put、raw get、raw delete、raw batch get、raw batch put、raw batch delete、raw scan等普通的KV操作
RxnKV是为了事务机制而设计的一系列操作

### TiKV源码剖析
```
    pub trait Engine: Send + Clone + 'static {
       type Snap: Snapshot;
       fn async_write(&self, ctx: &Contect, batch: Vec<Modify>, callback: Callback<()>) -> Result<()>;
       fn async_snapshot(&self, ctx: &Context, callback: Callback<Self::Snap>) -> Result<()>;
    }
```
