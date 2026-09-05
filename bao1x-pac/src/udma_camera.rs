#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_rx_saddr: RegRxSaddr,
    reg_rx_size: RegRxSize,
    reg_rx_cfg: RegRxCfg,
    _reserved3: [u8; 0x14],
    reg_cam_cfg_glob: RegCamCfgGlob,
    reg_cam_cfg_ll: RegCamCfgLl,
    reg_cam_cfg_ur: RegCamCfgUr,
    reg_cam_cfg_size: RegCamCfgSize,
    reg_cam_cfg_filter: RegCamCfgFilter,
    reg_cam_vsync_polarity: RegCamVsyncPolarity,
}
impl RegisterBlock {
    #[doc = "0x00 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x20 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_cfg_glob(&self) -> &RegCamCfgGlob {
        &self.reg_cam_cfg_glob
    }
    #[doc = "0x24 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_cfg_ll(&self) -> &RegCamCfgLl {
        &self.reg_cam_cfg_ll
    }
    #[doc = "0x28 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_cfg_ur(&self) -> &RegCamCfgUr {
        &self.reg_cam_cfg_ur
    }
    #[doc = "0x2c - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_cfg_size(&self) -> &RegCamCfgSize {
        &self.reg_cam_cfg_size
    }
    #[doc = "0x30 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_cfg_filter(&self) -> &RegCamCfgFilter {
        &self.reg_cam_cfg_filter
    }
    #[doc = "0x34 - See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cam_vsync_polarity(&self) -> &RegCamVsyncPolarity {
        &self.reg_cam_vsync_polarity
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_CAM_CFG_GLOB (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_glob::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_glob::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_cfg_glob`] module"]
#[doc(alias = "REG_CAM_CFG_GLOB")]
pub type RegCamCfgGlob = crate::Reg<reg_cam_cfg_glob::RegCamCfgGlobSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_cfg_glob;
#[doc = "REG_CAM_CFG_LL (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_ll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_ll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_cfg_ll`] module"]
#[doc(alias = "REG_CAM_CFG_LL")]
pub type RegCamCfgLl = crate::Reg<reg_cam_cfg_ll::RegCamCfgLlSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_cfg_ll;
#[doc = "REG_CAM_CFG_UR (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_ur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_ur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_cfg_ur`] module"]
#[doc(alias = "REG_CAM_CFG_UR")]
pub type RegCamCfgUr = crate::Reg<reg_cam_cfg_ur::RegCamCfgUrSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_cfg_ur;
#[doc = "REG_CAM_CFG_SIZE (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_cfg_size`] module"]
#[doc(alias = "REG_CAM_CFG_SIZE")]
pub type RegCamCfgSize = crate::Reg<reg_cam_cfg_size::RegCamCfgSizeSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_cfg_size;
#[doc = "REG_CAM_CFG_FILTER (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_cfg_filter`] module"]
#[doc(alias = "REG_CAM_CFG_FILTER")]
pub type RegCamCfgFilter = crate::Reg<reg_cam_cfg_filter::RegCamCfgFilterSpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_cfg_filter;
#[doc = "REG_CAM_VSYNC_POLARITY (rw) register accessor: See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_vsync_polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_vsync_polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cam_vsync_polarity`] module"]
#[doc(alias = "REG_CAM_VSYNC_POLARITY")]
pub type RegCamVsyncPolarity = crate::Reg<reg_cam_vsync_polarity::RegCamVsyncPolaritySpec>;
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__"]
pub mod reg_cam_vsync_polarity;
