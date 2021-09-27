# Collector问题想法
1. ![img_10.png](img_10.png)
2. ![img_11.png](img_11.png)
3. ![img_13.png](img_13.png)
4. ![img_12.png](img_12.png)


# Python项目总结
# 目前项目
1. 脚本单独去执行   

#### Source: 
* Amazon Athena
* agora
* huaweicloud
* fengkongcloud
* tencentcloud
* amazon aws

## 模块划分

![img_2.png](img_2.png)
* Usage Module  获取使用数据情况模块
  1. (定时)获取数据
    请求时的signature构建过程
    *. let result = 使用hmac库用sha1根据sk和具体的消息内容取hash并取摘要digest
    *. base64 urlsafe_b64encode(result).decode()
  2. 数据清理
  3. 数据落库
* src Module 处理数据并生成相关报告，比如dataFrame图像写入到excel中
* sgt_email Module 负责给对应人员发邮件的模块
* db Module 数据库处理相关模块
* data Module 获取具体花销的模块
* config Module 配置相关模块


#### 所使用的服务
1. TiDB
   全新的一栈式实时 HTAP (Hybrid Transactional/Analytical Processing)数据库
   特点:
      * 基于分布式架构，支持弹性扩容，可按需扩展吞吐或存储，便于应对高并发
      * 参考了Google Spanner和F1的设计，F1建立在Spanner之上     
           * F1 Goal
             1. 无需应用程序更改即可重新分片和重新平衡
             2. ACID 
           * Spanner Goal
             1. 管理跨数据中心复制的数据
             2. 重新分片和重新平衡数据
             3. 自动跨机器迁移数据
      * TiDB使用Raft一致性协议来同步数据，对于异地多活的场景比较好
      * TiDB提供完整的分布式事务
        1. 乐观事务
        2. 悲观事务  默认采用这个
        3. 事务大小限制
        4. 事务隔离级别采用可重复读
      * TiDB架构分析  上层分为TiDB和TiKV，TiDB对应的是Google F1，是一层无状态的SQL Layer，
        兼容绝大多数MySQL语法，对外暴露Mysql协议，负责解析用户的SQL语句，生成分布式的QueryPlan，
        翻译成底层Key Value操作发送给TiKV，TiKV是真正存储数据的地方，对应的是Google Spanner，
        TiKV是一个分布式Key Value数据库，支持弹性水平扩展，自动的灾难恢复和故障转移以及ACID跨行事务。
        TiKV不像是HBase和BigTable依赖底层的分布式系统，在灵活性上更好
        ![img_5.png](img_5.png)
        1. TiDB Server负责接受SQL请求，处理Sql相关的逻辑，并通过PD找到存储计算所需数据的TiKV地址，
           与TiKV交互获取数据，最终返回结果。TiDB Server是无状态的，其本身不存储数据，只负责计算，
           可以水平扩展
        2. PD(Placement Driver) Server是整个集群的管理模块，一是负责存储集群的元信息，某个Key存储在那个TiKV节点上，
           二是对TiKV集群进行调度和负载均衡，比如数据的迁移和raft group leader的迁移  三是分配全局唯一且递增的事务id
           PD是一个集群需要部署奇数个节点，一般线上至少部署三个
        3. TiKV server底层依赖于RocksDB，Facebook开源的单机KV存储引擎，负责存储数据，从外部看是一个分布式的提供事务的key value存储引擎，
           存储数据的基本单位是Region，每个region负责存储一个key range，每个TiKV节点负责多个Region，TiKV使用Raft协议做复制，保证数据的
           一致性和容灾，副本以region为单位进行管理，不同节点上多个region构成一个raft group，互为副本，数据在多个TiKV之间的负载由PD调度，
           以Region为单位进行调度。通过raft，TikV变成了一个分布式的Key-Value存储，少数几台机器宕机也能通过原生的Raft协议自动把副本补全，
           可以做到业务无感知。每一段region的最大大小设置为96MB。以region为单位做Raft的复制和成员管理。
        4. TiKV以region为单位做数据的复制，也就是一个Region的数据会保存为多个副本，每一个副本叫做一个replica，replica之间通过raft来保持
           数据的一致，一个region的多个replica会保存在不同的节点上，构成一个raft group，其中一个replica会作为这个group的leader，其他的
           replica会作为follower，默认情况所有的读和写都是通过leader进行，写操作从leader上写完再复制给follower
           ![img_7.png](img_7.png)
           该种设计结构使其具备了容灾能力
        5. TikV通过在key后面添加版本号实现MVCC
        6. TiKV事务采用的是Google在BigTable中使用的实物模型 Percolator
           ![img_6.png](img_6.png)  
        7. RocksDB：提供键值存储与读写功能的LSM-tree(Log Structured Merge Tree)架构引擎，用户写入的键值会先写入磁盘上的WAL(write ahead log),然后再写入内存中的跳表，
           在RocksDB中该跳表称之为MemTable，
           ![img_9.png](img_9.png)
        8. 内部存储引擎分为TiKV和TiFlash
           这两者具体区别
           TiFlash是TiDB对于AP(Analytical Processing)扩展,通过Raft Learner协议，从TiKV同步传过来的
           TiFlash与TiKV的最大区别，TiFlash借助ClickHouse向量化引擎，采用向量化存储，在计算上继承了它高性能的优点
           TiKV则是列式存储，数据写入慢，但是比较适合OLAP系统，读其中某一列的数据相当于
        9. TiDB Engine
        ```
        pub trait Engine: Send + Clone + 'static {
             type Snap: Snapshot;
             fn async_write(&self, ctx: &Contect, batch: Vec<Modify>, callback: Callback<()>) -> Result<()>;
             fn async_snapshot(&self, ctx: &Context, callback: Callback<Self::Snap>) -> Result<()>;
        }
        ```
        实现了上述Engine Trait都可以作为TiKV的底层存储引擎，目前现在包含Rocksdb_engine,btree_engine,raftkv
        目前默认使用的是RaftKV引擎，这里的async_write成功之后意味着raft协议下的write已经完成
        ``` 
          impl<E: Engine> Storage<E> {
               pub fn async_raw_put(
                 &self,
                 ctx: Context,
                 cf: String,
                 key: Vec<u8>,
                  value: Vec<u8>,
                  callback: Callback<()>,
                ) -> Result<()> {
               // Omit some limit checks about key and value here...
               self.engine.async_write(
               &ctx,
               vec![Modify::Put(
                   Self::rawkv_cf(&cf),
                   Key::from_encoded(key),
                   value,
               )],
               Box::new(|(_, res)| callback(res.map_err(Error::from))),
               )?;
               Ok(())
               }
          }
        ```
   性能对比:
      TPC-H衡量OLAP(online analytical processing)的性能    
      ![img_8.png](img_8.png)  
   
2. pandas::DateFrame
   数据可视化存储到excel中
   ![img_4.png](img_4.png)
3. boto amazon service client
 
### 目前可优化的地方
1可以将更多的配置相关抽到配置文件中，而不是硬编码在代码里，更直观些且便于维护  
 ---
 ![img.png](img.png)
 ---
 ![img_1.png](img_1.png)
2.![img_3.png](img_3.png)
   * 使用python协程替代线程执行io式任务
   * Class名和方法名可以进一步改善
3.不同脚本之间api目前是通过main来进行调用
4.整个项目可以更加工程化一些

#### 使用想法
1. jython调用对应的python代码获取对应的数据
2. 通过django或者flask将数据提供给新工程
3. 在现有python的基础上做成一套健壮的服务
4. 

# 对于可观测平台和成本平台的想法
