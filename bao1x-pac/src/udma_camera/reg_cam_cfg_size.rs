#[doc = "Register `REG_CAM_CFG_SIZE` reader"]
pub type R = crate::R<RegCamCfgSizeSpec>;
#[doc = "Register `REG_CAM_CFG_SIZE` writer"]
pub type W = crate::W<RegCamCfgSizeSpec>;
#[doc = "Field `r_cam_cfg_size` reader - r_cam_cfg_size"]
pub type RCamCfgSizeR = crate::FieldReader<u32>;
#[doc = "Field `r_cam_cfg_size` writer - r_cam_cfg_size"]
pub type RCamCfgSizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_cam_cfg_size"]
    #[inline(always)]
    pub fn r_cam_cfg_size(&self) -> RCamCfgSizeR {
        RCamCfgSizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_cam_cfg_size"]
    #[inline(always)]
    pub fn r_cam_cfg_size(&mut self) -> RCamCfgSizeW<'_, RegCamCfgSizeSpec> {
        RCamCfgSizeW::new(self, 0)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamCfgSizeSpec;
impl crate::RegisterSpec for RegCamCfgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_cfg_size::R`](R) reader structure"]
impl crate::Readable for RegCamCfgSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_cfg_size::W`](W) writer structure"]
impl crate::Writable for RegCamCfgSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_CFG_SIZE to value 0"]
impl crate::Resettable for RegCamCfgSizeSpec {}
