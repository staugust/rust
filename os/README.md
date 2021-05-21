# how to translate bootimage.bin to bootimage.vdi

```bash
dd if=bootimage.bin of=bootimage.pad.bin bs=1m conv=sync
VBoxManage convertdd bootimage.pad.bin bootimage.vdi --format VDI
```
