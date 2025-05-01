# 功能总结

- `sys_linkat`：在`vfs.rs`中，先通过`old_filename`找到源文件的inode，将inode的`nlink`加一。再修改`root_inode`，添加新的Entry，使用上面的inode_id。
- `sys_linkat`：在`vfs.rs`中，先通过`filename`找到源文件的inode。如果找到，将inode的`nlink`减一。再修改`root_inode`，将原来DirEntry的位置写0。
- `sys_stat`：在`vfs.rs`中，通过`read_disk_inode()`获得ino，nlink以及通过`is_file()`判断是文件还是目录。


# [简答题](https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter6/4exercise.html#id4)
## Chaper6
1. root inode是根目录，相当于Linux操作系统中的`/`。如果损坏将无法访问文件系统中的所有文件甚至于整个操作系统都无法加载。

## Chapter7
1. `ls -l | grep xxx`；`cat file.txt | wc -l`。
2. 使用消息队列，批量向进程发送消息，有操作系统统筹而不是进程机制操作。


# [荣誉准则](https://learningos.cn/rCore-Tutorial-Guide-2025S/honorcode.html)

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
<center>-</center>

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
<center>-</center>

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。