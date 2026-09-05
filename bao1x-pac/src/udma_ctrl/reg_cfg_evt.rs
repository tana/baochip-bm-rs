#[doc = "Register `REG_CFG_EVT` reader"]
pub type R = crate::R<RegCfgEvtSpec>;
#[doc = "Register `REG_CFG_EVT` writer"]
pub type W = crate::W<RegCfgEvtSpec>;
#[doc = "Field `r_cmp_evt_0` reader - r_cmp_evt_0"]
pub type RCmpEvt0R = crate::FieldReader;
#[doc = "Field `r_cmp_evt_0` writer - r_cmp_evt_0"]
pub type RCmpEvt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cmp_evt_1` reader - r_cmp_evt_1"]
pub type RCmpEvt1R = crate::FieldReader;
#[doc = "Field `r_cmp_evt_1` writer - r_cmp_evt_1"]
pub type RCmpEvt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cmp_evt_2` reader - r_cmp_evt_2"]
pub type RCmpEvt2R = crate::FieldReader;
#[doc = "Field `r_cmp_evt_2` writer - r_cmp_evt_2"]
pub type RCmpEvt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_cmp_evt_3` reader - r_cmp_evt_3"]
pub type RCmpEvt3R = crate::FieldReader;
#[doc = "Field `r_cmp_evt_3` writer - r_cmp_evt_3"]
pub type RCmpEvt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_cmp_evt_0"]
    #[inline(always)]
    pub fn r_cmp_evt_0(&self) -> RCmpEvt0R {
        RCmpEvt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - r_cmp_evt_1"]
    #[inline(always)]
    pub fn r_cmp_evt_1(&self) -> RCmpEvt1R {
        RCmpEvt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - r_cmp_evt_2"]
    #[inline(always)]
    pub fn r_cmp_evt_2(&self) -> RCmpEvt2R {
        RCmpEvt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - r_cmp_evt_3"]
    #[inline(always)]
    pub fn r_cmp_evt_3(&self) -> RCmpEvt3R {
        RCmpEvt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_cmp_evt_0"]
    #[inline(always)]
    pub fn r_cmp_evt_0(&mut self) -> RCmpEvt0W<'_, RegCfgEvtSpec> {
        RCmpEvt0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - r_cmp_evt_1"]
    #[inline(always)]
    pub fn r_cmp_evt_1(&mut self) -> RCmpEvt1W<'_, RegCfgEvtSpec> {
        RCmpEvt1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - r_cmp_evt_2"]
    #[inline(always)]
    pub fn r_cmp_evt_2(&mut self) -> RCmpEvt2W<'_, RegCfgEvtSpec> {
        RCmpEvt2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - r_cmp_evt_3"]
    #[inline(always)]
    pub fn r_cmp_evt_3(&mut self) -> RCmpEvt3W<'_, RegCfgEvtSpec> {
        RCmpEvt3W::new(self, 24)
    }
}
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cfg_evt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cfg_evt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCfgEvtSpec;
impl crate::RegisterSpec for RegCfgEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cfg_evt::R`](R) reader structure"]
impl crate::Readable for RegCfgEvtSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cfg_evt::W`](W) writer structure"]
impl crate::Writable for RegCfgEvtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CFG_EVT to value 0"]
impl crate::Resettable for RegCfgEvtSpec {}
