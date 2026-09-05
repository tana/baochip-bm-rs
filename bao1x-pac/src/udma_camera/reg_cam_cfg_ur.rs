#[doc = "Register `REG_CAM_CFG_UR` reader"]
pub type R = crate::R<RegCamCfgUrSpec>;
#[doc = "Register `REG_CAM_CFG_UR` writer"]
pub type W = crate::W<RegCamCfgUrSpec>;
#[doc = "Field `r_cam_cfg_ur` reader - r_cam_cfg_ur"]
pub type RCamCfgUrR = crate::FieldReader<u32>;
#[doc = "Field `r_cam_cfg_ur` writer - r_cam_cfg_ur"]
pub type RCamCfgUrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_cam_cfg_ur"]
    #[inline(always)]
    pub fn r_cam_cfg_ur(&self) -> RCamCfgUrR {
        RCamCfgUrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_cam_cfg_ur"]
    #[inline(always)]
    pub fn r_cam_cfg_ur(&mut self) -> RCamCfgUrW<'_, RegCamCfgUrSpec> {
        RCamCfgUrW::new(self, 0)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_cfg_ur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_cfg_ur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamCfgUrSpec;
impl crate::RegisterSpec for RegCamCfgUrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_cfg_ur::R`](R) reader structure"]
impl crate::Readable for RegCamCfgUrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_cfg_ur::W`](W) writer structure"]
impl crate::Writable for RegCamCfgUrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_CFG_UR to value 0"]
impl crate::Resettable for RegCamCfgUrSpec {}
