#[doc = "Register `REG_CAM_CFG_FILTER` reader"]
pub type R = crate::R<RegCamCfgFilterSpec>;
#[doc = "Register `REG_CAM_CFG_FILTER` writer"]
pub type W = crate::W<RegCamCfgFilterSpec>;
#[doc = "Field `r_cam_cfg_filter` reader - r_cam_cfg_filter"]
pub type RCamCfgFilterR = crate::FieldReader<u32>;
#[doc = "Field `r_cam_cfg_filter` writer - r_cam_cfg_filter"]
pub type RCamCfgFilterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_cam_cfg_filter"]
    #[inline(always)]
    pub fn r_cam_cfg_filter(&self) -> RCamCfgFilterR {
        RCamCfgFilterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_cam_cfg_filter"]
    #[inline(always)]
    pub fn r_cam_cfg_filter(&mut self) -> RCamCfgFilterW<'_, RegCamCfgFilterSpec> {
        RCamCfgFilterW::new(self, 0)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamCfgFilterSpec;
impl crate::RegisterSpec for RegCamCfgFilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_cfg_filter::R`](R) reader structure"]
impl crate::Readable for RegCamCfgFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_cfg_filter::W`](W) writer structure"]
impl crate::Writable for RegCamCfgFilterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_CFG_FILTER to value 0"]
impl crate::Resettable for RegCamCfgFilterSpec {}
