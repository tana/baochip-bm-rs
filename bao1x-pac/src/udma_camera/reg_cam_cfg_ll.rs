#[doc = "Register `REG_CAM_CFG_LL` reader"]
pub type R = crate::R<RegCamCfgLlSpec>;
#[doc = "Register `REG_CAM_CFG_LL` writer"]
pub type W = crate::W<RegCamCfgLlSpec>;
#[doc = "Field `r_cam_cfg_ll` reader - r_cam_cfg_ll"]
pub type RCamCfgLlR = crate::FieldReader<u32>;
#[doc = "Field `r_cam_cfg_ll` writer - r_cam_cfg_ll"]
pub type RCamCfgLlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_cam_cfg_ll"]
    #[inline(always)]
    pub fn r_cam_cfg_ll(&self) -> RCamCfgLlR {
        RCamCfgLlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_cam_cfg_ll"]
    #[inline(always)]
    pub fn r_cam_cfg_ll(&mut self) -> RCamCfgLlW<'_, RegCamCfgLlSpec> {
        RCamCfgLlW::new(self, 0)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_ll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_ll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamCfgLlSpec;
impl crate::RegisterSpec for RegCamCfgLlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_cfg_ll::R`](R) reader structure"]
impl crate::Readable for RegCamCfgLlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_cfg_ll::W`](W) writer structure"]
impl crate::Writable for RegCamCfgLlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_CFG_LL to value 0"]
impl crate::Resettable for RegCamCfgLlSpec {}
