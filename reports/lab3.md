# 功能总结

- `sys_spawn`：通过`get_app_data_by_name()`获得`elf_data`，然后获得当前TCB，通过`elf_data`创建新的TCB并添加到当前TCB的`children`中，将新的Task添加到队列，返回新的tid。
- stride 调度算法：在TCBInner中添加新字段`prio`和`stride`，初始分别为16和0。修改`manager.rs`中的`fetch()`方法，先从ready_queue中找到stride最小的任务，并将该任务从ready_queue中移除，并将stride增加pass。


# [简答题](https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter4/7exercise.html#id3)

- 不是。u8最大为255，255 + 10发生一溢出变成4了，所以还是p1继续执行。

- $pass = \frac{BigStride}{priority} \leq \frac{BigStride}{2}$，每次切换进程时这个进程最多前进$\frac{BigStride}{2}$，然后就会切换为别的进程，差距会越来越小，所以最多是$\frac{BigStride}{2}$。
- 
```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let diff = (self.0  - other.0) as i64;
        if diff < 0 {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }

```


# [荣誉准则](https://learningos.cn/rCore-Tutorial-Guide-2025S/honorcode.html)

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
<center>-</center>

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
<center>-</center>

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。