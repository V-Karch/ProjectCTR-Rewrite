# ProjectCTR-Rewrite
This is a rewrite of the current state of [ProjectCTR](https://github.com/3DSGuy/Project_CTR) as of commit hash `6337c49a15cc7fcee9a3c9e44758dbce2393b26e`

## Roadmap

✅ => Ready for testing  
🟨 => In progress  
❌ => Not started  

├── accessdesc.rs ❌  
├── aes_keygen.rs ❌  
├── cardinfo.rs ❌  
├── certs.rs ❌  
├── cia_build.rs ❌  
├── cia.rs ❌  
├── cia_read.rs ❌  
├── code.rs ❌  
├── crr.rs 🟨  
├── crypto.rs ❌  
├── ctr_utils.rs 🟨  
├── desc ✅  
│   ├── desc.rs ✅  
│   ├── dev_sigdata.rs ✅  
│   └── presets.rs ✅  
├── elf.rs ❌  
├── exefs_build.rs ❌  
├── exefs.rs ❌  
├── exefs_read.rs ❌  
├── exheader_build.rs ❌  
├── exheader.rs ❌  
├── exheader_read.rs ❌  
├── keyset.rs ❌  
├── lib.rs ❌  
├── makerom.rs ❌ (contains int main() {})  
├── ncch_build.rs ❌  
├── ncch.rs ❌  
├── ncch_logo.rs ❌  
├── ncch_read.rs ❌  
├── ncsd_build.rs ❌  
├── ncsd.rs ❌  
├── ncsd_read.rs ❌  
├── oschar.rs ❌  
├── pki ✅  
│   ├── dev.rs ✅  
│   ├── dev_legacy.rs ✅  
│   ├── prod.h ✅  
│   ├── prod_legacy.h ✅  
│   ├── rsa_key.h ✅  
│   └── test.h ✅  
├── romfs.rs ❌  
├── romfs_fs.rs ❌  
├── romfs_gen.rs ❌  
├── romfs_import.rs ❌  
├── rsf_settings.rs ❌  
├── srl.rs ❌  
├── tik_build.rs ❌  
├── tik.rs ❌  
├── tik_read.rs ❌  
├── titleid.rs ❌  
├── tmd_build.rs ❌  
├── tmd.rs ❌  
├── tmd_read.rs ❌  
├── types.rs ✅  
├── user_settings.rs 🟨  
├── utils.rs 🟨  
└── yaml_parser.rs ❌  
