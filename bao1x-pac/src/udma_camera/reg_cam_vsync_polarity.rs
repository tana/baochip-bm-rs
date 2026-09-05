#[doc = "Register `REG_CAM_VSYNC_POLARITY` reader"]
pub type R = crate::R<RegCamVsyncPolaritySpec>;
#[doc = "Register `REG_CAM_VSYNC_POLARITY` writer"]
pub type W = crate::W<RegCamVsyncPolaritySpec>;
#[doc = "Field `r_cam_vsync_polarity` reader - r_cam_vsync_polarity"]
pub type RCamVsyncPolarityR = crate::BitReader;
#[doc = "Field `r_cam_vsync_polarity` writer - r_cam_vsync_polarity"]
pub type RCamVsyncPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cam_hsync_polarity` reader - r_cam_hsync_polarity"]
pub type RCamHsyncPolarityR = crate::BitReader;
#[doc = "Field `r_cam_hsync_polarity` writer - r_cam_hsync_polarity"]
pub type RCamHsyncPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_cam_vsync_polarity"]
    #[inline(always)]
    pub fn r_cam_vsync_polarity(&self) -> RCamVsyncPolarityR {
        RCamVsyncPolarityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_cam_hsync_polarity"]
    #[inline(always)]
    pub fn r_cam_hsync_polarity(&self) -> RCamHsyncPolarityR {
        RCamHsyncPolarityR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_cam_vsync_polarity"]
    #[inline(always)]
    pub fn r_cam_vsync_polarity(&mut self) -> RCamVsyncPolarityW<'_, RegCamVsyncPolaritySpec> {
        RCamVsyncPolarityW::new(self, 0)
    }
    #[doc = "Bit 1 - r_cam_hsync_polarity"]
    #[inline(always)]
    pub fn r_cam_hsync_polarity(&mut self) -> RCamHsyncPolarityW<'_, RegCamVsyncPolaritySpec> {
        RCamHsyncPolarityW::new(self, 1)
    }
}
#[doc = "See `camera_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/u dma/udma_camera/rtl/camera_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cam_vsync_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cam_vsync_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCamVsyncPolaritySpec;
impl crate::RegisterSpec for RegCamVsyncPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cam_vsync_polarity::R`](R) reader structure"]
impl crate::Readable for RegCamVsyncPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cam_vsync_polarity::W`](W) writer structure"]
impl crate::Writable for RegCamVsyncPolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAM_VSYNC_POLARITY to value 0"]
impl crate::Resettable for RegCamVsyncPolaritySpec {}
