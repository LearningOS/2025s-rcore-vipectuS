# 功能总结

在`sys_trace`函数中，先对`_trace_request`进行pattern match:
- 0 -> 使用unsafe直接对`_id`进行deref后返回位置上的值；
- 1 -> 使用unsafe对`_id`deref后在该位置写入_data；
- 2 -> 调用task/mod.rs中相应的函数；
- _ -> 返回-1。

为了统计每个task对系统调用的次数，选择在`TaskManagerInner`中创建一个新的二维数组`syscall_counts`，一维大小为`MAX_SYSCALL_NUM`，二维大小为`MAX_APP_NUM`，二者分别以全局常量的形式定义在config.rs中。

应对该数组的维护，在syscall/mod.rs的`syscall`函数开头调用了对应的函数，通过传入当前系统调用id来增加对应计数。

这只是一个简单的实现，没有考虑任何错误处理。在性能和空间权衡后选择直接使用一个二维数组储存系统调用次数。


# [简答题](https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter3/5exercise.html#id6)

1. `[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.`
   `[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2`
   
   出错行为：kernel打印了错误原因（PageFault和IllegalInstruction）并提示任务已被kernel杀死。从结果看相关程序最终没有被执行。

2. 1.分配Trap上下文之后内核栈栈顶（向下增长）；\_\_restore的出现目前永远与需要从内核栈恢复Trap上下文有关，两种使用场景分别为：`trap_handler`后返回用户态和第一次进入用户态`run_first_task`直接\_\_switch执行第一个应用时。
   
   2.sstatus：该Trap发生前Cpu处于哪个特权级；
     sepc：回到用户态后的程序从spec指示的地址开始继续执行；
    sscratch：在此时指向用户栈栈顶，在恢复Trap上下文并与sp交换后指向新的内核栈栈顶。
   
   3.x2：x2就是sp，此时指向内核栈栈顶，所有恢复Trap上下文的操作都依赖这个地址，只能最后恢复。在陷入Trap之前x2的值为用户栈栈顶，在\_\_alltraps中被保存为了sscratch，最后`csrrw sp, sscratch, sp`相当于恢复了x2。
	x4：除非我们手动出于一些特殊用途使用它，否则一般也不会被用到。在\_\_alltraps中就没有保存。
   
   4.sp：用户栈栈顶；sscratch：内核栈栈顶。
   
   5.sret。CPU 完成 Trap 处理准备返回的时候，需要通过一条 S 特权级的特权指令 sret 来完成，这一条指令具体完成以下功能：CPU 会将当前的特权级按照 sstatus 的 SPP 字段设置为 U 或者 S；
   
   6.sp：内核栈栈顶；sscratch：用户栈栈顶。
   
   7.call trap_handler之后


# [荣誉准则](https://learningos.cn/rCore-Tutorial-Guide-2025S/honorcode.html)

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
<center>-</center>

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
<center>-</center>

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。