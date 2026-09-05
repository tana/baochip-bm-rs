#[doc = "Register `REG_CAM_CFG_GLOB` reader"]
pub type R = crate::R<RegCamCfgGlobSpec>;
#[doc = "Register `REG_CAM_CFG_GLOB` writer"]
pub type W = crate::W<RegCamCfgGlobSpec>;
#[doc = "Field `r_cam_cfg` reader - r_cam_cfg"]
pub type RCamCfgR = crate::FieldReader<u32>;
#[doc = "Field `r_cam_cfg` writer - r_cam_cfg"]
pub type RCamCfgW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `cfg_cam_ip_en_i` reader - cfg_cam_ip_en_i"]
pub type CfgCamIpEnIR = crate::BitReader;
#[doc = "Field `cfg_cam_ip_en_i` writer - cfg_cam_ip_en_i"]
pub type CfgCamIpEnIW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - r_cam_cfg"]
    #[inline(always)]
    pub fn r_cam_cfg(&self) -> RCamCfgR {
        RCamCfgR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - cfg_cam_ip_en_i"]
    #[inline(always)]
    pub fn cfg_cam_ip_en_i(&self) -> CfgCamIpEnIR {
        CfgCamIpEnIR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - r_cam_cfg"]
    #[inline(always)]
    pub fn r_cam_cfg(&mut self) -> RCamCfgW<'_, RegCamCfgGlobSpec> {
        RCamCfgW::new(self, 0)
    }
    #[doc = "Bit 30 - cfg_cam_ip_en_i"]
    #[inline(always)]
    pub fn cfg_cam_ip_en_i(&mut self) -> CfgCamIpEnIW<'_, RegCamCfgGlobSpec> {
        CfgCamIpEnIW::new(self, 30)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_glob::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_glob::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamCfgGlobSpec;
impl crate::RegisterSpec for RegCamCfgGlobSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_cfg_glob::R`](R) reader structure"]
impl crate::Readable for RegCamCfgGlobSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_cfg_glob::W`](W) writer structure"]
impl crate::Writable for RegCamCfgGlobSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_CFG_GLOB to value 0"]
impl crate::Resettable for RegCamCfgGlobSpec {}
