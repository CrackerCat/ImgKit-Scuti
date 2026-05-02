## v1.2.5 更新日志

### 本次更新
- 修好 sparse 包裹的 F2FS 镜像解包：现在 `unpack` 能识别 `sparse_f2fs`，HyperOS 之类的 userdata.img 可以直接走全流程。
- 修好 `pack --type f2fs -S` 的 sparse 输出：之前这个标志被忽略只会写出 raw 镜像，现在会真正生成 Android Sparse 格式。

<details>
<summary>English Version</summary>

## v1.2.5 Changelog

### Highlights
- Fixed sparse-wrapped F2FS image unpacking: `unpack` now recognises `sparse_f2fs`, so userdata images such as those from HyperOS go through the full pipeline.
- Fixed `pack --type f2fs -S`: the sparse flag was previously ignored and produced raw images; the command now emits the proper Android Sparse format.

</details>
