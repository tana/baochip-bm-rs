#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_io: SfrIo,
    sfr_ar: SfrAr,
    _reserved2: [u8; 0x08],
    cr_ocr: CrOcr,
    cr_rdffthres: CrRdffthres,
    cr_rev: CrRev,
    cr_bacsa: CrBacsa,
    cr_baiofn_cfg_base_addr_io_func0: CrBaiofnCfgBaseAddrIoFunc0,
    cr_baiofn_cfg_base_addr_io_func1: CrBaiofnCfgBaseAddrIoFunc1,
    cr_baiofn_cfg_base_addr_io_func2: CrBaiofnCfgBaseAddrIoFunc2,
    cr_baiofn_cfg_base_addr_io_func3: CrBaiofnCfgBaseAddrIoFunc3,
    cr_baiofn_cfg_base_addr_io_func4: CrBaiofnCfgBaseAddrIoFunc4,
    cr_baiofn_cfg_base_addr_io_func5: CrBaiofnCfgBaseAddrIoFunc5,
    cr_baiofn_cfg_base_addr_io_func6: CrBaiofnCfgBaseAddrIoFunc6,
    cr_baiofn_cfg_base_addr_io_func7: CrBaiofnCfgBaseAddrIoFunc7,
    cr_fncisptr_cfg_reg_func_cis_ptr0: CrFncisptrCfgRegFuncCisPtr0,
    cr_fncisptr_cfg_reg_func_cis_ptr1: CrFncisptrCfgRegFuncCisPtr1,
    cr_fncisptr_cfg_reg_func_cis_ptr2: CrFncisptrCfgRegFuncCisPtr2,
    cr_fncisptr_cfg_reg_func_cis_ptr3: CrFncisptrCfgRegFuncCisPtr3,
    cr_fncisptr_cfg_reg_func_cis_ptr4: CrFncisptrCfgRegFuncCisPtr4,
    cr_fncisptr_cfg_reg_func_cis_ptr5: CrFncisptrCfgRegFuncCisPtr5,
    cr_fncisptr_cfg_reg_func_cis_ptr6: CrFncisptrCfgRegFuncCisPtr6,
    cr_fncisptr_cfg_reg_func_cis_ptr7: CrFncisptrCfgRegFuncCisPtr7,
    cr_fnextstdcode_cfg_reg_func_ext_std_code0: CrFnextstdcodeCfgRegFuncExtStdCode0,
    cr_fnextstdcode_cfg_reg_func_ext_std_code1: CrFnextstdcodeCfgRegFuncExtStdCode1,
    cr_fnextstdcode_cfg_reg_func_ext_std_code2: CrFnextstdcodeCfgRegFuncExtStdCode2,
    cr_fnextstdcode_cfg_reg_func_ext_std_code3: CrFnextstdcodeCfgRegFuncExtStdCode3,
    cr_fnextstdcode_cfg_reg_func_ext_std_code4: CrFnextstdcodeCfgRegFuncExtStdCode4,
    cr_fnextstdcode_cfg_reg_func_ext_std_code5: CrFnextstdcodeCfgRegFuncExtStdCode5,
    cr_fnextstdcode_cfg_reg_func_ext_std_code6: CrFnextstdcodeCfgRegFuncExtStdCode6,
    cr_fnextstdcode_cfg_reg_func_ext_std_code7: CrFnextstdcodeCfgRegFuncExtStdCode7,
    cr_write_protect: CrWriteProtect,
    cr_reg_dsr: CrRegDsr,
    cr_reg_cid_cfg_reg_cid0: CrRegCidCfgRegCid0,
    cr_reg_cid_cfg_reg_cid1: CrRegCidCfgRegCid1,
    cr_reg_cid_cfg_reg_cid2: CrRegCidCfgRegCid2,
    cr_reg_cid_cfg_reg_cid3: CrRegCidCfgRegCid3,
    cr_reg_csd_cfg_reg_csd0: CrRegCsdCfgRegCsd0,
    cr_reg_csd_cfg_reg_csd1: CrRegCsdCfgRegCsd1,
    cr_reg_csd_cfg_reg_csd2: CrRegCsdCfgRegCsd2,
    cr_reg_csd_cfg_reg_csd3: CrRegCsdCfgRegCsd3,
    cr_reg_scr_cfg_reg_scr0: CrRegScrCfgRegScr0,
    cr_reg_scr_cfg_reg_scr1: CrRegScrCfgRegScr1,
    cr_reg_sd_status_cfg_reg_sd_status0: CrRegSdStatusCfgRegSdStatus0,
    cr_reg_sd_status_cfg_reg_sd_status1: CrRegSdStatusCfgRegSdStatus1,
    cr_reg_sd_status_cfg_reg_sd_status2: CrRegSdStatusCfgRegSdStatus2,
    cr_reg_sd_status_cfg_reg_sd_status3: CrRegSdStatusCfgRegSdStatus3,
    cr_reg_sd_status_cfg_reg_sd_status4: CrRegSdStatusCfgRegSdStatus4,
    cr_reg_sd_status_cfg_reg_sd_status5: CrRegSdStatusCfgRegSdStatus5,
    cr_reg_sd_status_cfg_reg_sd_status6: CrRegSdStatusCfgRegSdStatus6,
    cr_reg_sd_status_cfg_reg_sd_status7: CrRegSdStatusCfgRegSdStatus7,
    cr_reg_sd_status_cfg_reg_sd_status8: CrRegSdStatusCfgRegSdStatus8,
    cr_reg_sd_status_cfg_reg_sd_status9: CrRegSdStatusCfgRegSdStatus9,
    cr_reg_sd_status_cfg_reg_sd_status10: CrRegSdStatusCfgRegSdStatus10,
    cr_reg_sd_status_cfg_reg_sd_status11: CrRegSdStatusCfgRegSdStatus11,
    cr_reg_sd_status_cfg_reg_sd_status12: CrRegSdStatusCfgRegSdStatus12,
    cr_reg_sd_status_cfg_reg_sd_status13: CrRegSdStatusCfgRegSdStatus13,
    cr_reg_sd_status_cfg_reg_sd_status14: CrRegSdStatusCfgRegSdStatus14,
    cr_reg_sd_status_cfg_reg_sd_status15: CrRegSdStatusCfgRegSdStatus15,
    _reserved58: [u8; 0x10],
    cr_base_addr_mem_func_cfg_base_addr_mem_func0: CrBaseAddrMemFuncCfgBaseAddrMemFunc0,
    cr_base_addr_mem_func_cfg_base_addr_mem_func1: CrBaseAddrMemFuncCfgBaseAddrMemFunc1,
    cr_base_addr_mem_func_cfg_base_addr_mem_func2: CrBaseAddrMemFuncCfgBaseAddrMemFunc2,
    cr_base_addr_mem_func_cfg_base_addr_mem_func3: CrBaseAddrMemFuncCfgBaseAddrMemFunc3,
    cr_base_addr_mem_func_cfg_base_addr_mem_func4: CrBaseAddrMemFuncCfgBaseAddrMemFunc4,
    cr_base_addr_mem_func_cfg_base_addr_mem_func5: CrBaseAddrMemFuncCfgBaseAddrMemFunc5,
    cr_base_addr_mem_func_cfg_base_addr_mem_func6: CrBaseAddrMemFuncCfgBaseAddrMemFunc6,
    cr_base_addr_mem_func_cfg_base_addr_mem_func7: CrBaseAddrMemFuncCfgBaseAddrMemFunc7,
    cr_base_addr_mem_func_cfg_base_addr_mem_func8: CrBaseAddrMemFuncCfgBaseAddrMemFunc8,
    cr_base_addr_mem_func_cfg_base_addr_mem_func9: CrBaseAddrMemFuncCfgBaseAddrMemFunc9,
    cr_base_addr_mem_func_cfg_base_addr_mem_func10: CrBaseAddrMemFuncCfgBaseAddrMemFunc10,
    cr_base_addr_mem_func_cfg_base_addr_mem_func11: CrBaseAddrMemFuncCfgBaseAddrMemFunc11,
    cr_base_addr_mem_func_cfg_base_addr_mem_func12: CrBaseAddrMemFuncCfgBaseAddrMemFunc12,
    cr_base_addr_mem_func_cfg_base_addr_mem_func13: CrBaseAddrMemFuncCfgBaseAddrMemFunc13,
    cr_base_addr_mem_func_cfg_base_addr_mem_func14: CrBaseAddrMemFuncCfgBaseAddrMemFunc14,
    cr_base_addr_mem_func_cfg_base_addr_mem_func15: CrBaseAddrMemFuncCfgBaseAddrMemFunc15,
    cr_base_addr_mem_func_cfg_base_addr_mem_func16: CrBaseAddrMemFuncCfgBaseAddrMemFunc16,
    cr_base_addr_mem_func_cfg_base_addr_mem_func17: CrBaseAddrMemFuncCfgBaseAddrMemFunc17,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode0,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode1,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode5,
    cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6:
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6,
    _reserved83: [u8; 0x04],
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code0:
        CrRegFuncManufactCodeCfgRegFuncManufactCode0,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code1:
        CrRegFuncManufactCodeCfgRegFuncManufactCode1,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code2:
        CrRegFuncManufactCodeCfgRegFuncManufactCode2,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code3:
        CrRegFuncManufactCodeCfgRegFuncManufactCode3,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code4:
        CrRegFuncManufactCodeCfgRegFuncManufactCode4,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code5:
        CrRegFuncManufactCodeCfgRegFuncManufactCode5,
    cr_reg_func_manufact_code_cfg_reg_func_manufact_code6:
        CrRegFuncManufactCodeCfgRegFuncManufactCode6,
    _reserved90: [u8; 0x04],
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info0:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo0,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info1:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo1,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info2:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo2,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info3:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo3,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info4:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo4,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info5:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo5,
    cr_reg_func_manufact_info_cfg_reg_func_manufact_info6:
        CrRegFuncManufactInfoCfgRegFuncManufactInfo6,
    _reserved97: [u8; 0x04],
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode0,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode1,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode2,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode4,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode5,
    cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6:
        CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode6,
    _reserved104: [u8; 0x04],
    cr_reg_func_info_cfg_reg_func_info0: CrRegFuncInfoCfgRegFuncInfo0,
    cr_reg_func_info_cfg_reg_func_info1: CrRegFuncInfoCfgRegFuncInfo1,
    cr_reg_func_info_cfg_reg_func_info2: CrRegFuncInfoCfgRegFuncInfo2,
    cr_reg_func_info_cfg_reg_func_info3: CrRegFuncInfoCfgRegFuncInfo3,
    cr_reg_func_info_cfg_reg_func_info4: CrRegFuncInfoCfgRegFuncInfo4,
    cr_reg_func_info_cfg_reg_func_info5: CrRegFuncInfoCfgRegFuncInfo5,
    cr_reg_func_info_cfg_reg_func_info6: CrRegFuncInfoCfgRegFuncInfo6,
    _reserved111: [u8; 0x0c],
    cr_reg_uhs_1_support: CrRegUhs1Support,
}
impl RegisterBlock {
    #[doc = "0x00 - See `sddc.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L113>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_io(&self) -> &SfrIo {
        &self.sfr_io
    }
    #[doc = "0x04 - See `sddc.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L114>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar(&self) -> &SfrAr {
        &self.sfr_ar
    }
    #[doc = "0x10 - See `sddc.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L116>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_ocr(&self) -> &CrOcr {
        &self.cr_ocr
    }
    #[doc = "0x14 - See `sddc.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_rdffthres(&self) -> &CrRdffthres {
        &self.cr_rdffthres
    }
    #[doc = "0x18 - See `sddc.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L118>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_rev(&self) -> &CrRev {
        &self.cr_rev
    }
    #[doc = "0x1c - See `sddc.sv#L120 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L120>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_bacsa(&self) -> &CrBacsa {
        &self.cr_bacsa
    }
    #[doc = "0x20 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func0(&self) -> &CrBaiofnCfgBaseAddrIoFunc0 {
        &self.cr_baiofn_cfg_base_addr_io_func0
    }
    #[doc = "0x24 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func1(&self) -> &CrBaiofnCfgBaseAddrIoFunc1 {
        &self.cr_baiofn_cfg_base_addr_io_func1
    }
    #[doc = "0x28 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func2(&self) -> &CrBaiofnCfgBaseAddrIoFunc2 {
        &self.cr_baiofn_cfg_base_addr_io_func2
    }
    #[doc = "0x2c - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func3(&self) -> &CrBaiofnCfgBaseAddrIoFunc3 {
        &self.cr_baiofn_cfg_base_addr_io_func3
    }
    #[doc = "0x30 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func4(&self) -> &CrBaiofnCfgBaseAddrIoFunc4 {
        &self.cr_baiofn_cfg_base_addr_io_func4
    }
    #[doc = "0x34 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func5(&self) -> &CrBaiofnCfgBaseAddrIoFunc5 {
        &self.cr_baiofn_cfg_base_addr_io_func5
    }
    #[doc = "0x38 - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func6(&self) -> &CrBaiofnCfgBaseAddrIoFunc6 {
        &self.cr_baiofn_cfg_base_addr_io_func6
    }
    #[doc = "0x3c - See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_baiofn_cfg_base_addr_io_func7(&self) -> &CrBaiofnCfgBaseAddrIoFunc7 {
        &self.cr_baiofn_cfg_base_addr_io_func7
    }
    #[doc = "0x40 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr0(&self) -> &CrFncisptrCfgRegFuncCisPtr0 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr0
    }
    #[doc = "0x44 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr1(&self) -> &CrFncisptrCfgRegFuncCisPtr1 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr1
    }
    #[doc = "0x48 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr2(&self) -> &CrFncisptrCfgRegFuncCisPtr2 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr2
    }
    #[doc = "0x4c - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr3(&self) -> &CrFncisptrCfgRegFuncCisPtr3 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr3
    }
    #[doc = "0x50 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr4(&self) -> &CrFncisptrCfgRegFuncCisPtr4 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr4
    }
    #[doc = "0x54 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr5(&self) -> &CrFncisptrCfgRegFuncCisPtr5 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr5
    }
    #[doc = "0x58 - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr6(&self) -> &CrFncisptrCfgRegFuncCisPtr6 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr6
    }
    #[doc = "0x5c - See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fncisptr_cfg_reg_func_cis_ptr7(&self) -> &CrFncisptrCfgRegFuncCisPtr7 {
        &self.cr_fncisptr_cfg_reg_func_cis_ptr7
    }
    #[doc = "0x60 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code0(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode0 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code0
    }
    #[doc = "0x64 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code1(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode1 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code1
    }
    #[doc = "0x68 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code2(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode2 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code2
    }
    #[doc = "0x6c - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code3(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode3 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code3
    }
    #[doc = "0x70 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code4(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode4 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code4
    }
    #[doc = "0x74 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code5(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode5 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code5
    }
    #[doc = "0x78 - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code6(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode6 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code6
    }
    #[doc = "0x7c - See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_fnextstdcode_cfg_reg_func_ext_std_code7(
        &self,
    ) -> &CrFnextstdcodeCfgRegFuncExtStdCode7 {
        &self.cr_fnextstdcode_cfg_reg_func_ext_std_code7
    }
    #[doc = "0x80 - See `sddc.sv#L134 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L134>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_write_protect(&self) -> &CrWriteProtect {
        &self.cr_write_protect
    }
    #[doc = "0x84 - See `sddc.sv#L135 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L135>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_dsr(&self) -> &CrRegDsr {
        &self.cr_reg_dsr
    }
    #[doc = "0x88 - See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_cid_cfg_reg_cid0(&self) -> &CrRegCidCfgRegCid0 {
        &self.cr_reg_cid_cfg_reg_cid0
    }
    #[doc = "0x8c - See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_cid_cfg_reg_cid1(&self) -> &CrRegCidCfgRegCid1 {
        &self.cr_reg_cid_cfg_reg_cid1
    }
    #[doc = "0x90 - See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_cid_cfg_reg_cid2(&self) -> &CrRegCidCfgRegCid2 {
        &self.cr_reg_cid_cfg_reg_cid2
    }
    #[doc = "0x94 - See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_cid_cfg_reg_cid3(&self) -> &CrRegCidCfgRegCid3 {
        &self.cr_reg_cid_cfg_reg_cid3
    }
    #[doc = "0x98 - See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_csd_cfg_reg_csd0(&self) -> &CrRegCsdCfgRegCsd0 {
        &self.cr_reg_csd_cfg_reg_csd0
    }
    #[doc = "0x9c - See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_csd_cfg_reg_csd1(&self) -> &CrRegCsdCfgRegCsd1 {
        &self.cr_reg_csd_cfg_reg_csd1
    }
    #[doc = "0xa0 - See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_csd_cfg_reg_csd2(&self) -> &CrRegCsdCfgRegCsd2 {
        &self.cr_reg_csd_cfg_reg_csd2
    }
    #[doc = "0xa4 - See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_csd_cfg_reg_csd3(&self) -> &CrRegCsdCfgRegCsd3 {
        &self.cr_reg_csd_cfg_reg_csd3
    }
    #[doc = "0xa8 - See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_scr_cfg_reg_scr0(&self) -> &CrRegScrCfgRegScr0 {
        &self.cr_reg_scr_cfg_reg_scr0
    }
    #[doc = "0xac - See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_scr_cfg_reg_scr1(&self) -> &CrRegScrCfgRegScr1 {
        &self.cr_reg_scr_cfg_reg_scr1
    }
    #[doc = "0xb0 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status0(&self) -> &CrRegSdStatusCfgRegSdStatus0 {
        &self.cr_reg_sd_status_cfg_reg_sd_status0
    }
    #[doc = "0xb4 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status1(&self) -> &CrRegSdStatusCfgRegSdStatus1 {
        &self.cr_reg_sd_status_cfg_reg_sd_status1
    }
    #[doc = "0xb8 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status2(&self) -> &CrRegSdStatusCfgRegSdStatus2 {
        &self.cr_reg_sd_status_cfg_reg_sd_status2
    }
    #[doc = "0xbc - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status3(&self) -> &CrRegSdStatusCfgRegSdStatus3 {
        &self.cr_reg_sd_status_cfg_reg_sd_status3
    }
    #[doc = "0xc0 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status4(&self) -> &CrRegSdStatusCfgRegSdStatus4 {
        &self.cr_reg_sd_status_cfg_reg_sd_status4
    }
    #[doc = "0xc4 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status5(&self) -> &CrRegSdStatusCfgRegSdStatus5 {
        &self.cr_reg_sd_status_cfg_reg_sd_status5
    }
    #[doc = "0xc8 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status6(&self) -> &CrRegSdStatusCfgRegSdStatus6 {
        &self.cr_reg_sd_status_cfg_reg_sd_status6
    }
    #[doc = "0xcc - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status7(&self) -> &CrRegSdStatusCfgRegSdStatus7 {
        &self.cr_reg_sd_status_cfg_reg_sd_status7
    }
    #[doc = "0xd0 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status8(&self) -> &CrRegSdStatusCfgRegSdStatus8 {
        &self.cr_reg_sd_status_cfg_reg_sd_status8
    }
    #[doc = "0xd4 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status9(&self) -> &CrRegSdStatusCfgRegSdStatus9 {
        &self.cr_reg_sd_status_cfg_reg_sd_status9
    }
    #[doc = "0xd8 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status10(&self) -> &CrRegSdStatusCfgRegSdStatus10 {
        &self.cr_reg_sd_status_cfg_reg_sd_status10
    }
    #[doc = "0xdc - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status11(&self) -> &CrRegSdStatusCfgRegSdStatus11 {
        &self.cr_reg_sd_status_cfg_reg_sd_status11
    }
    #[doc = "0xe0 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status12(&self) -> &CrRegSdStatusCfgRegSdStatus12 {
        &self.cr_reg_sd_status_cfg_reg_sd_status12
    }
    #[doc = "0xe4 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status13(&self) -> &CrRegSdStatusCfgRegSdStatus13 {
        &self.cr_reg_sd_status_cfg_reg_sd_status13
    }
    #[doc = "0xe8 - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status14(&self) -> &CrRegSdStatusCfgRegSdStatus14 {
        &self.cr_reg_sd_status_cfg_reg_sd_status14
    }
    #[doc = "0xec - See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_sd_status_cfg_reg_sd_status15(&self) -> &CrRegSdStatusCfgRegSdStatus15 {
        &self.cr_reg_sd_status_cfg_reg_sd_status15
    }
    #[doc = "0x100 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func0(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc0 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func0
    }
    #[doc = "0x104 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func1(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc1 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func1
    }
    #[doc = "0x108 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func2(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc2 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func2
    }
    #[doc = "0x10c - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func3(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc3 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func3
    }
    #[doc = "0x110 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func4(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc4 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func4
    }
    #[doc = "0x114 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func5(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc5 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func5
    }
    #[doc = "0x118 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func6(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc6 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func6
    }
    #[doc = "0x11c - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func7(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc7 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func7
    }
    #[doc = "0x120 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func8(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc8 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func8
    }
    #[doc = "0x124 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func9(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc9 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func9
    }
    #[doc = "0x128 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func10(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc10 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func10
    }
    #[doc = "0x12c - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func11(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc11 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func11
    }
    #[doc = "0x130 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func12(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc12 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func12
    }
    #[doc = "0x134 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func13(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc13 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func13
    }
    #[doc = "0x138 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func14(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc14 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func14
    }
    #[doc = "0x13c - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func15(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc15 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func15
    }
    #[doc = "0x140 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func16(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc16 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func16
    }
    #[doc = "0x144 - See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_base_addr_mem_func_cfg_base_addr_mem_func17(
        &self,
    ) -> &CrBaseAddrMemFuncCfgBaseAddrMemFunc17 {
        &self.cr_base_addr_mem_func_cfg_base_addr_mem_func17
    }
    #[doc = "0x148 - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode0 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0
    }
    #[doc = "0x14c - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode1 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1
    }
    #[doc = "0x150 - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2
    }
    #[doc = "0x154 - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3
    }
    #[doc = "0x158 - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4
    }
    #[doc = "0x15c - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode5 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5
    }
    #[doc = "0x160 - See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6(
        &self,
    ) -> &CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6 {
        &self.cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6
    }
    #[doc = "0x168 - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code0(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode0 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code0
    }
    #[doc = "0x16c - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code1(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode1 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code1
    }
    #[doc = "0x170 - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code2(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode2 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code2
    }
    #[doc = "0x174 - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code3(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode3 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code3
    }
    #[doc = "0x178 - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code4(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode4 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code4
    }
    #[doc = "0x17c - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code5(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode5 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code5
    }
    #[doc = "0x180 - See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_code_cfg_reg_func_manufact_code6(
        &self,
    ) -> &CrRegFuncManufactCodeCfgRegFuncManufactCode6 {
        &self.cr_reg_func_manufact_code_cfg_reg_func_manufact_code6
    }
    #[doc = "0x188 - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info0(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo0 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info0
    }
    #[doc = "0x18c - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info1(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo1 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info1
    }
    #[doc = "0x190 - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info2(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo2 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info2
    }
    #[doc = "0x194 - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info3(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo3 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info3
    }
    #[doc = "0x198 - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info4(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo4 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info4
    }
    #[doc = "0x19c - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info5(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo5 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info5
    }
    #[doc = "0x1a0 - See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_manufact_info_cfg_reg_func_manufact_info6(
        &self,
    ) -> &CrRegFuncManufactInfoCfgRegFuncManufactInfo6 {
        &self.cr_reg_func_manufact_info_cfg_reg_func_manufact_info6
    }
    #[doc = "0x1a8 - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode0 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0
    }
    #[doc = "0x1ac - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode1 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1
    }
    #[doc = "0x1b0 - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode2 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2
    }
    #[doc = "0x1b4 - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3
    }
    #[doc = "0x1b8 - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode4 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4
    }
    #[doc = "0x1bc - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode5 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5
    }
    #[doc = "0x1c0 - See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6(
        &self,
    ) -> &CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode6 {
        &self.cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6
    }
    #[doc = "0x1c8 - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info0(&self) -> &CrRegFuncInfoCfgRegFuncInfo0 {
        &self.cr_reg_func_info_cfg_reg_func_info0
    }
    #[doc = "0x1cc - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info1(&self) -> &CrRegFuncInfoCfgRegFuncInfo1 {
        &self.cr_reg_func_info_cfg_reg_func_info1
    }
    #[doc = "0x1d0 - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info2(&self) -> &CrRegFuncInfoCfgRegFuncInfo2 {
        &self.cr_reg_func_info_cfg_reg_func_info2
    }
    #[doc = "0x1d4 - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info3(&self) -> &CrRegFuncInfoCfgRegFuncInfo3 {
        &self.cr_reg_func_info_cfg_reg_func_info3
    }
    #[doc = "0x1d8 - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info4(&self) -> &CrRegFuncInfoCfgRegFuncInfo4 {
        &self.cr_reg_func_info_cfg_reg_func_info4
    }
    #[doc = "0x1dc - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info5(&self) -> &CrRegFuncInfoCfgRegFuncInfo5 {
        &self.cr_reg_func_info_cfg_reg_func_info5
    }
    #[doc = "0x1e0 - See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_func_info_cfg_reg_func_info6(&self) -> &CrRegFuncInfoCfgRegFuncInfo6 {
        &self.cr_reg_func_info_cfg_reg_func_info6
    }
    #[doc = "0x1f0 - See `sddc.sv#L159 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L159>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_reg_uhs_1_support(&self) -> &CrRegUhs1Support {
        &self.cr_reg_uhs_1_support
    }
}
#[doc = "SFR_IO (rw) register accessor: See `sddc.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L113>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_io`] module"]
#[doc(alias = "SFR_IO")]
pub type SfrIo = crate::Reg<sfr_io::SfrIoSpec>;
#[doc = "See `sddc.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L113>`__ (line numbers are approximate)"]
pub mod sfr_io;
#[doc = "SFR_AR (rw) register accessor: See `sddc.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L114>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar`] module"]
#[doc(alias = "SFR_AR")]
pub type SfrAr = crate::Reg<sfr_ar::SfrArSpec>;
#[doc = "See `sddc.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L114>`__ (line numbers are approximate)"]
pub mod sfr_ar;
#[doc = "CR_OCR (rw) register accessor: See `sddc.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L116>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ocr`] module"]
#[doc(alias = "CR_OCR")]
pub type CrOcr = crate::Reg<cr_ocr::CrOcrSpec>;
#[doc = "See `sddc.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L116>`__ (line numbers are approximate)"]
pub mod cr_ocr;
#[doc = "CR_RDFFTHRES (rw) register accessor: See `sddc.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rdffthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rdffthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_rdffthres`] module"]
#[doc(alias = "CR_RDFFTHRES")]
pub type CrRdffthres = crate::Reg<cr_rdffthres::CrRdffthresSpec>;
#[doc = "See `sddc.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L117>`__ (line numbers are approximate)"]
pub mod cr_rdffthres;
#[doc = "CR_REV (rw) register accessor: See `sddc.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L118>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_rev`] module"]
#[doc(alias = "CR_REV")]
pub type CrRev = crate::Reg<cr_rev::CrRevSpec>;
#[doc = "See `sddc.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L118>`__ (line numbers are approximate)"]
pub mod cr_rev;
#[doc = "CR_BACSA (rw) register accessor: See `sddc.sv#L120 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L120>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_bacsa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_bacsa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_bacsa`] module"]
#[doc(alias = "CR_BACSA")]
pub type CrBacsa = crate::Reg<cr_bacsa::CrBacsaSpec>;
#[doc = "See `sddc.sv#L120 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L120>`__ (line numbers are approximate)"]
pub mod cr_bacsa;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC0 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func0`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC0")]
pub type CrBaiofnCfgBaseAddrIoFunc0 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func0::CrBaiofnCfgBaseAddrIoFunc0Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func0;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC1 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func1`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC1")]
pub type CrBaiofnCfgBaseAddrIoFunc1 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func1::CrBaiofnCfgBaseAddrIoFunc1Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func1;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func2`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2")]
pub type CrBaiofnCfgBaseAddrIoFunc2 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func2::CrBaiofnCfgBaseAddrIoFunc2Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func2;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC3 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func3`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC3")]
pub type CrBaiofnCfgBaseAddrIoFunc3 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func3::CrBaiofnCfgBaseAddrIoFunc3Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func3;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC4 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func4`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC4")]
pub type CrBaiofnCfgBaseAddrIoFunc4 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func4::CrBaiofnCfgBaseAddrIoFunc4Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func4;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC5 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func5`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC5")]
pub type CrBaiofnCfgBaseAddrIoFunc5 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func5::CrBaiofnCfgBaseAddrIoFunc5Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func5;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func6`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6")]
pub type CrBaiofnCfgBaseAddrIoFunc6 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func6::CrBaiofnCfgBaseAddrIoFunc6Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func6;
#[doc = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC7 (rw) register accessor: See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_baiofn_cfg_base_addr_io_func7`] module"]
#[doc(alias = "CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC7")]
pub type CrBaiofnCfgBaseAddrIoFunc7 =
    crate::Reg<cr_baiofn_cfg_base_addr_io_func7::CrBaiofnCfgBaseAddrIoFunc7Spec>;
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)"]
pub mod cr_baiofn_cfg_base_addr_io_func7;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR0 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr0`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR0")]
pub type CrFncisptrCfgRegFuncCisPtr0 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr0::CrFncisptrCfgRegFuncCisPtr0Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr0;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR1 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr1`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR1")]
pub type CrFncisptrCfgRegFuncCisPtr1 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr1::CrFncisptrCfgRegFuncCisPtr1Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr1;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr2`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2")]
pub type CrFncisptrCfgRegFuncCisPtr2 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr2::CrFncisptrCfgRegFuncCisPtr2Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr2;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR3 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr3`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR3")]
pub type CrFncisptrCfgRegFuncCisPtr3 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr3::CrFncisptrCfgRegFuncCisPtr3Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr3;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr4`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4")]
pub type CrFncisptrCfgRegFuncCisPtr4 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr4::CrFncisptrCfgRegFuncCisPtr4Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr4;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR5 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr5`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR5")]
pub type CrFncisptrCfgRegFuncCisPtr5 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr5::CrFncisptrCfgRegFuncCisPtr5Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr5;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR6 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr6`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR6")]
pub type CrFncisptrCfgRegFuncCisPtr6 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr6::CrFncisptrCfgRegFuncCisPtr6Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr6;
#[doc = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7 (rw) register accessor: See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fncisptr_cfg_reg_func_cis_ptr7`] module"]
#[doc(alias = "CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7")]
pub type CrFncisptrCfgRegFuncCisPtr7 =
    crate::Reg<cr_fncisptr_cfg_reg_func_cis_ptr7::CrFncisptrCfgRegFuncCisPtr7Spec>;
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)"]
pub mod cr_fncisptr_cfg_reg_func_cis_ptr7;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE0 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code0`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE0")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode0 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code0::CrFnextstdcodeCfgRegFuncExtStdCode0Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code0;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE1 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code1`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE1")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode1 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code1::CrFnextstdcodeCfgRegFuncExtStdCode1Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code1;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code2`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode2 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code2::CrFnextstdcodeCfgRegFuncExtStdCode2Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code2;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE3 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code3`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE3")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode3 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code3::CrFnextstdcodeCfgRegFuncExtStdCode3Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code3;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code4`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode4 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code4::CrFnextstdcodeCfgRegFuncExtStdCode4Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code4;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code5`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode5 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code5::CrFnextstdcodeCfgRegFuncExtStdCode5Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code5;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE6 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code6`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE6")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode6 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code6::CrFnextstdcodeCfgRegFuncExtStdCode6Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code6;
#[doc = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE7 (rw) register accessor: See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_fnextstdcode_cfg_reg_func_ext_std_code7`] module"]
#[doc(alias = "CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE7")]
pub type CrFnextstdcodeCfgRegFuncExtStdCode7 =
    crate::Reg<cr_fnextstdcode_cfg_reg_func_ext_std_code7::CrFnextstdcodeCfgRegFuncExtStdCode7Spec>;
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)"]
pub mod cr_fnextstdcode_cfg_reg_func_ext_std_code7;
#[doc = "CR_WRITE_PROTECT (rw) register accessor: See `sddc.sv#L134 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L134>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_write_protect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_write_protect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_write_protect`] module"]
#[doc(alias = "CR_WRITE_PROTECT")]
pub type CrWriteProtect = crate::Reg<cr_write_protect::CrWriteProtectSpec>;
#[doc = "See `sddc.sv#L134 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L134>`__ (line numbers are approximate)"]
pub mod cr_write_protect;
#[doc = "CR_REG_DSR (rw) register accessor: See `sddc.sv#L135 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L135>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_dsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_dsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_dsr`] module"]
#[doc(alias = "CR_REG_DSR")]
pub type CrRegDsr = crate::Reg<cr_reg_dsr::CrRegDsrSpec>;
#[doc = "See `sddc.sv#L135 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L135>`__ (line numbers are approximate)"]
pub mod cr_reg_dsr;
#[doc = "CR_REG_CID_CFG_REG_CID0 (rw) register accessor: See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_cid_cfg_reg_cid0`] module"]
#[doc(alias = "CR_REG_CID_CFG_REG_CID0")]
pub type CrRegCidCfgRegCid0 = crate::Reg<cr_reg_cid_cfg_reg_cid0::CrRegCidCfgRegCid0Spec>;
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
pub mod cr_reg_cid_cfg_reg_cid0;
#[doc = "CR_REG_CID_CFG_REG_CID1 (rw) register accessor: See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_cid_cfg_reg_cid1`] module"]
#[doc(alias = "CR_REG_CID_CFG_REG_CID1")]
pub type CrRegCidCfgRegCid1 = crate::Reg<cr_reg_cid_cfg_reg_cid1::CrRegCidCfgRegCid1Spec>;
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
pub mod cr_reg_cid_cfg_reg_cid1;
#[doc = "CR_REG_CID_CFG_REG_CID2 (rw) register accessor: See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_cid_cfg_reg_cid2`] module"]
#[doc(alias = "CR_REG_CID_CFG_REG_CID2")]
pub type CrRegCidCfgRegCid2 = crate::Reg<cr_reg_cid_cfg_reg_cid2::CrRegCidCfgRegCid2Spec>;
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
pub mod cr_reg_cid_cfg_reg_cid2;
#[doc = "CR_REG_CID_CFG_REG_CID3 (rw) register accessor: See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_cid_cfg_reg_cid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_cid_cfg_reg_cid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_cid_cfg_reg_cid3`] module"]
#[doc(alias = "CR_REG_CID_CFG_REG_CID3")]
pub type CrRegCidCfgRegCid3 = crate::Reg<cr_reg_cid_cfg_reg_cid3::CrRegCidCfgRegCid3Spec>;
#[doc = "See `sddc.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L136>`__ (line numbers are approximate)"]
pub mod cr_reg_cid_cfg_reg_cid3;
#[doc = "CR_REG_CSD_CFG_REG_CSD0 (rw) register accessor: See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_csd_cfg_reg_csd0`] module"]
#[doc(alias = "CR_REG_CSD_CFG_REG_CSD0")]
pub type CrRegCsdCfgRegCsd0 = crate::Reg<cr_reg_csd_cfg_reg_csd0::CrRegCsdCfgRegCsd0Spec>;
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
pub mod cr_reg_csd_cfg_reg_csd0;
#[doc = "CR_REG_CSD_CFG_REG_CSD1 (rw) register accessor: See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_csd_cfg_reg_csd1`] module"]
#[doc(alias = "CR_REG_CSD_CFG_REG_CSD1")]
pub type CrRegCsdCfgRegCsd1 = crate::Reg<cr_reg_csd_cfg_reg_csd1::CrRegCsdCfgRegCsd1Spec>;
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
pub mod cr_reg_csd_cfg_reg_csd1;
#[doc = "CR_REG_CSD_CFG_REG_CSD2 (rw) register accessor: See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_csd_cfg_reg_csd2`] module"]
#[doc(alias = "CR_REG_CSD_CFG_REG_CSD2")]
pub type CrRegCsdCfgRegCsd2 = crate::Reg<cr_reg_csd_cfg_reg_csd2::CrRegCsdCfgRegCsd2Spec>;
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
pub mod cr_reg_csd_cfg_reg_csd2;
#[doc = "CR_REG_CSD_CFG_REG_CSD3 (rw) register accessor: See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_csd_cfg_reg_csd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_csd_cfg_reg_csd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_csd_cfg_reg_csd3`] module"]
#[doc(alias = "CR_REG_CSD_CFG_REG_CSD3")]
pub type CrRegCsdCfgRegCsd3 = crate::Reg<cr_reg_csd_cfg_reg_csd3::CrRegCsdCfgRegCsd3Spec>;
#[doc = "See `sddc.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L137>`__ (line numbers are approximate)"]
pub mod cr_reg_csd_cfg_reg_csd3;
#[doc = "CR_REG_SCR_CFG_REG_SCR0 (rw) register accessor: See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_scr_cfg_reg_scr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_scr_cfg_reg_scr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_scr_cfg_reg_scr0`] module"]
#[doc(alias = "CR_REG_SCR_CFG_REG_SCR0")]
pub type CrRegScrCfgRegScr0 = crate::Reg<cr_reg_scr_cfg_reg_scr0::CrRegScrCfgRegScr0Spec>;
#[doc = "See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)"]
pub mod cr_reg_scr_cfg_reg_scr0;
#[doc = "CR_REG_SCR_CFG_REG_SCR1 (rw) register accessor: See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_scr_cfg_reg_scr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_scr_cfg_reg_scr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_scr_cfg_reg_scr1`] module"]
#[doc(alias = "CR_REG_SCR_CFG_REG_SCR1")]
pub type CrRegScrCfgRegScr1 = crate::Reg<cr_reg_scr_cfg_reg_scr1::CrRegScrCfgRegScr1Spec>;
#[doc = "See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)"]
pub mod cr_reg_scr_cfg_reg_scr1;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS0 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status0`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS0")]
pub type CrRegSdStatusCfgRegSdStatus0 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status0::CrRegSdStatusCfgRegSdStatus0Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status0;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS1 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status1`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS1")]
pub type CrRegSdStatusCfgRegSdStatus1 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status1::CrRegSdStatusCfgRegSdStatus1Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status1;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS2 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status2`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS2")]
pub type CrRegSdStatusCfgRegSdStatus2 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status2::CrRegSdStatusCfgRegSdStatus2Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status2;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS3 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status3`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS3")]
pub type CrRegSdStatusCfgRegSdStatus3 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status3::CrRegSdStatusCfgRegSdStatus3Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status3;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS4 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status4`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS4")]
pub type CrRegSdStatusCfgRegSdStatus4 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status4::CrRegSdStatusCfgRegSdStatus4Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status4;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS5 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status5`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS5")]
pub type CrRegSdStatusCfgRegSdStatus5 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status5::CrRegSdStatusCfgRegSdStatus5Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status5;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS6 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status6`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS6")]
pub type CrRegSdStatusCfgRegSdStatus6 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status6::CrRegSdStatusCfgRegSdStatus6Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status6;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS7 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status7`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS7")]
pub type CrRegSdStatusCfgRegSdStatus7 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status7::CrRegSdStatusCfgRegSdStatus7Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status7;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS8 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status8`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS8")]
pub type CrRegSdStatusCfgRegSdStatus8 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status8::CrRegSdStatusCfgRegSdStatus8Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status8;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS9 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status9`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS9")]
pub type CrRegSdStatusCfgRegSdStatus9 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status9::CrRegSdStatusCfgRegSdStatus9Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status9;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS10 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status10`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS10")]
pub type CrRegSdStatusCfgRegSdStatus10 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status10::CrRegSdStatusCfgRegSdStatus10Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status10;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS11 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status11`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS11")]
pub type CrRegSdStatusCfgRegSdStatus11 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status11::CrRegSdStatusCfgRegSdStatus11Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status11;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS12 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status12`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS12")]
pub type CrRegSdStatusCfgRegSdStatus12 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status12::CrRegSdStatusCfgRegSdStatus12Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status12;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS13 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status13`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS13")]
pub type CrRegSdStatusCfgRegSdStatus13 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status13::CrRegSdStatusCfgRegSdStatus13Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status13;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS14 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status14`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS14")]
pub type CrRegSdStatusCfgRegSdStatus14 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status14::CrRegSdStatusCfgRegSdStatus14Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status14;
#[doc = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS15 (rw) register accessor: See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_sd_status_cfg_reg_sd_status15`] module"]
#[doc(alias = "CR_REG_SD_STATUS_CFG_REG_SD_STATUS15")]
pub type CrRegSdStatusCfgRegSdStatus15 =
    crate::Reg<cr_reg_sd_status_cfg_reg_sd_status15::CrRegSdStatusCfgRegSdStatus15Spec>;
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)"]
pub mod cr_reg_sd_status_cfg_reg_sd_status15;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC0 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func0`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC0")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc0 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func0::CrBaseAddrMemFuncCfgBaseAddrMemFunc0Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func0;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC1 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func1`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC1")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc1 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func1::CrBaseAddrMemFuncCfgBaseAddrMemFunc1Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func1;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC2 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func2`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC2")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc2 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func2::CrBaseAddrMemFuncCfgBaseAddrMemFunc2Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func2;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC3 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func3`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC3")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc3 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func3::CrBaseAddrMemFuncCfgBaseAddrMemFunc3Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func3;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC4 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func4`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC4")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc4 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func4::CrBaseAddrMemFuncCfgBaseAddrMemFunc4Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func4;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC5 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func5`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC5")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc5 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func5::CrBaseAddrMemFuncCfgBaseAddrMemFunc5Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func5;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC6 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func6`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC6")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc6 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func6::CrBaseAddrMemFuncCfgBaseAddrMemFunc6Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func6;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC7 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func7`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC7")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc7 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func7::CrBaseAddrMemFuncCfgBaseAddrMemFunc7Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func7;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC8 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func8`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC8")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc8 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func8::CrBaseAddrMemFuncCfgBaseAddrMemFunc8Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func8;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC9 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func9`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC9")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc9 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func9::CrBaseAddrMemFuncCfgBaseAddrMemFunc9Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func9;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC10 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func10`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC10")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc10 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func10::CrBaseAddrMemFuncCfgBaseAddrMemFunc10Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func10;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func11`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc11 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func11::CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func11;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC12 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func12`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC12")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc12 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func12::CrBaseAddrMemFuncCfgBaseAddrMemFunc12Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func12;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func13`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc13 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func13::CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func13;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC14 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func14`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC14")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc14 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func14::CrBaseAddrMemFuncCfgBaseAddrMemFunc14Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func14;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC15 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func15`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC15")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc15 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func15::CrBaseAddrMemFuncCfgBaseAddrMemFunc15Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func15;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC16 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func16`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC16")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc16 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func16::CrBaseAddrMemFuncCfgBaseAddrMemFunc16Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func16;
#[doc = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC17 (rw) register accessor: See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_base_addr_mem_func_cfg_base_addr_mem_func17`] module"]
#[doc(alias = "CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC17")]
pub type CrBaseAddrMemFuncCfgBaseAddrMemFunc17 = crate::Reg<
    cr_base_addr_mem_func_cfg_base_addr_mem_func17::CrBaseAddrMemFuncCfgBaseAddrMemFunc17Spec,
>;
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)"]
pub mod cr_base_addr_mem_func_cfg_base_addr_mem_func17;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE0 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE0")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode0 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode0Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code0;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE1 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE1")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode1 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode1Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code1;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE5 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE5")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode5 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode5Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code5;
#[doc = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6 (rw) register accessor: See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6")]
pub type CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6 = crate :: Reg < cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6 :: CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec > ;
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE0 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code0`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE0")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode0 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code0 :: CrRegFuncManufactCodeCfgRegFuncManufactCode0Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code0;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE1 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code1`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE1")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode1 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code1 :: CrRegFuncManufactCodeCfgRegFuncManufactCode1Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code1;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code2`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode2 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code2 :: CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code2;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE3 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code3`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE3")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode3 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code3 :: CrRegFuncManufactCodeCfgRegFuncManufactCode3Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code3;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE4 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code4`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE4")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode4 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code4 :: CrRegFuncManufactCodeCfgRegFuncManufactCode4Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code4;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE5 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code5`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE5")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode5 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code5 :: CrRegFuncManufactCodeCfgRegFuncManufactCode5Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code5;
#[doc = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE6 (rw) register accessor: See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_code_cfg_reg_func_manufact_code6`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE6")]
pub type CrRegFuncManufactCodeCfgRegFuncManufactCode6 = crate :: Reg < cr_reg_func_manufact_code_cfg_reg_func_manufact_code6 :: CrRegFuncManufactCodeCfgRegFuncManufactCode6Spec > ;
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_code_cfg_reg_func_manufact_code6;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO0 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info0`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO0")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo0 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info0 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo0Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info0;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info1`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo1 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info1 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info1;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO2 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info2`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO2")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo2 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info2 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo2Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info2;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO3 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info3`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO3")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo3 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info3 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo3Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info3;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info4`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo4 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info4 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info4;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info5`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo5 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info5 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info5;
#[doc = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO6 (rw) register accessor: See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_manufact_info_cfg_reg_func_manufact_info6`] module"]
#[doc(alias = "CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO6")]
pub type CrRegFuncManufactInfoCfgRegFuncManufactInfo6 = crate :: Reg < cr_reg_func_manufact_info_cfg_reg_func_manufact_info6 :: CrRegFuncManufactInfoCfgRegFuncManufactInfo6Spec > ;
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)"]
pub mod cr_reg_func_manufact_info_cfg_reg_func_manufact_info6;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE0 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE0")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode0 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode0Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code0;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE1 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE1")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode1 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode1Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code1;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE2 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE2")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode2 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode2Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code2;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE4 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE4")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode4 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode4Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code4;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE5 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE5")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode5 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode5Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code5;
#[doc = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE6 (rw) register accessor: See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6`] module"]
#[doc(alias = "CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE6")]
pub type CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode6 = crate :: Reg < cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6 :: CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode6Spec > ;
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)"]
pub mod cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code6;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO0 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info0`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO0")]
pub type CrRegFuncInfoCfgRegFuncInfo0 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info0::CrRegFuncInfoCfgRegFuncInfo0Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info0;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info1`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1")]
pub type CrRegFuncInfoCfgRegFuncInfo1 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info1::CrRegFuncInfoCfgRegFuncInfo1Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info1;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO2 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info2`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO2")]
pub type CrRegFuncInfoCfgRegFuncInfo2 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info2::CrRegFuncInfoCfgRegFuncInfo2Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info2;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO3 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info3`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO3")]
pub type CrRegFuncInfoCfgRegFuncInfo3 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info3::CrRegFuncInfoCfgRegFuncInfo3Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info3;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO4 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info4`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO4")]
pub type CrRegFuncInfoCfgRegFuncInfo4 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info4::CrRegFuncInfoCfgRegFuncInfo4Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info4;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info5`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5")]
pub type CrRegFuncInfoCfgRegFuncInfo5 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info5::CrRegFuncInfoCfgRegFuncInfo5Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info5;
#[doc = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6 (rw) register accessor: See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_func_info_cfg_reg_func_info6`] module"]
#[doc(alias = "CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6")]
pub type CrRegFuncInfoCfgRegFuncInfo6 =
    crate::Reg<cr_reg_func_info_cfg_reg_func_info6::CrRegFuncInfoCfgRegFuncInfo6Spec>;
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)"]
pub mod cr_reg_func_info_cfg_reg_func_info6;
#[doc = "CR_REG_UHS_1_SUPPORT (rw) register accessor: See `sddc.sv#L159 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L159>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_uhs_1_support::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_uhs_1_support::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_reg_uhs_1_support`] module"]
#[doc(alias = "CR_REG_UHS_1_SUPPORT")]
pub type CrRegUhs1Support = crate::Reg<cr_reg_uhs_1_support::CrRegUhs1SupportSpec>;
#[doc = "See `sddc.sv#L159 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L159>`__ (line numbers are approximate)"]
pub mod cr_reg_uhs_1_support;
