# OctoMap - An Efficient Probabilistic 3D Mapping Framework Based on Octrees. Implemented in pure Rust.

---

## 简介/Abstract
OctoMap是一个基于八叉树的3D占用网格地图库，特别适用于机器人领域。它提供了Rust中的数据结构和映射算法，旨在满足以下需求：

- **全3D模型**：能够模拟任意环境，无需预先假设。表示模型包括占用区域和自由空间，未知区域隐含在地图中。
- **可更新**：可以随时添加新信息或传感器读数。建模和更新以概率方式进行，适用于传感器噪声或环境动态变化。
- **灵活**：地图范围无需预先知道，可根据需要动态扩展。地图是多分辨率的，适用于不同级别的规划器。
- **紧凑**：地图在内存和磁盘上高效存储，可生成压缩文件，便于机器人之间的交换。

## 参考原理/Reference
```BibTex
@ARTICLE{hornung13auro,
  author = {Armin Hornung and Kai M. Wurm and Maren Bennewitz and Cyrill Stachniss and Wolfram Burgard},
  title = {{OctoMap}: An Efficient Probabilistic {3D} Mapping Framework Based on Octrees},
  journal = {Autonomous Robots},
  year = 2013,
  url = {https://octomap.github.io},
  doi = {10.1007/s10514-012-9321-0},
  note = {Software available at \url{https://octomap.github.io}}
}
```
