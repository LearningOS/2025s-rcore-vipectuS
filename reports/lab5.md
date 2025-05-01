# 功能总结

- `enable_deadlock_detect`：给`ProcessControlBlockInner`添加新字段`enable_deadlock_detect: bool`，根据_enabled判断是否开启死锁检测。因为`ProcessControlBlockInner`适合PCB绑定的所以该选项只会影响该线程。
- 死锁检测：选择了现场构建向量的方式检测死锁。
  Mutex：实现中忽略了旋锁，Mutex现在需要实现`Any` trait并实现`as_any()`。在检测中获取所有非旋锁，`avaliable[]`根据锁是否被占用决定、`allocation[][]`根据`BlockingMutex`中的新字段`owner_id`决定，该字段为当前锁的拥有着的tid，在`lock()`和`unlock()`中维护。`need[][]`根据`wait_queue`得到。最后执行该算法。
  Semaphore：构建过程类似，除了在算法执行阶段有更严格的判定。因为`work[]`可以为负数。在判定时需要满足`need[i][j] == 0`或者`work[j] >= 0`并且`need[i,j] <= work[j]`。


# [简答题](https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter8/5exercise.html#id4)

1. 所有线程的用户栈，对应每个线程本身的TCB，线程在任务调度队列中的引用。
   调度器的就绪队列，Mutex、Semaphore中的等待队列。需要回收，不过会在PCB生命周期结束时自动被回收。
2. 区别：第一种锁没有获得锁之前不会返回`lock()`。释放锁时第一种锁会直接释放，第二种锁会检查等待队列，如果有等待的线程不会释放锁而会唤醒等待的线程。
   问题：第二种锁光`add_task(waking_task)`是不够的，因为虽然将该任务添加到了就绪队列却没有释放锁，该任务仍然不能获取锁导致死锁。


# [荣誉准则](https://learningos.cn/rCore-Tutorial-Guide-2025S/honorcode.html)

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
<center>-</center>

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
<center>-</center>

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。