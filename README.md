# learning_rust

因为最近在学习 rust，并且也跟着 [程序人生](https://www.youtube.com/channel/UCclzXvMOdinN4JHv1uLRv1g) 的视频学习 rust，这位大神在里面说了几个学习语言的方式；本质上想学到新的知识，就必须要让它与你之前的知识进行关联，所以你可以尝试做一些你比较熟悉的工程，慢慢的让它变成你的技能的一部分; 所以后期我会将我学习 rust 的所有的工程都比较完成的发布出来；基本上的方式是先按照自己的思路去尝试写，然后编译不过或者设计不合理在慢慢的修改; 前期估计会按照大神的项目为主，慢慢的我会转到自己的项目上来;目上来;;

## 1. 可能想道的几个不错的项目

* 分布式 kv 存储，基于内存 or rocksdb; 慢慢会在内部加入 raft 等一些组件来保证副本之间的一致性; 
* rust 的 raft 协议的实现；不会类似于 tikv 那种要求很高的，只不过将基本的流程能玩起来;
* stream 清洗数据的平台，写这个主要是为了玩，目前用 golang 在实现，现在用 rust 实现一边主要是为了对基础知识的掌握;
* pow 上课要求的
* threadpool
* learning_rust 过程中的一些代码小片段也会记录
* 配置中心
* b+ tree 实现，主要是一直都没实现过，想实现看看