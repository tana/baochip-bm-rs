#[doc = "Register `REG_EVENT_CFG` reader"]
pub type R = crate::R<RegEventCfgSpec>;
#[doc = "Register `REG_EVENT_CFG` writer"]
pub type W = crate::W<RegEventCfgSpec>;
#[doc = "Field `r_event_sel_0` reader - r_event_sel_0"]
pub type REventSel0R = crate::FieldReader;
#[doc = "Field `r_event_sel_0` writer - r_event_sel_0"]
pub type REventSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_event_sel_1` reader - r_event_sel_1"]
pub type REventSel1R = crate::FieldReader;
#[doc = "Field `r_event_sel_1` writer - r_event_sel_1"]
pub type REventSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_event_sel_2` reader - r_event_sel_2"]
pub type REventSel2R = crate::FieldReader;
#[doc = "Field `r_event_sel_2` writer - r_event_sel_2"]
pub type REventSel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_event_sel_3` reader - r_event_sel_3"]
pub type REventSel3R = crate::FieldReader;
#[doc = "Field `r_event_sel_3` writer - r_event_sel_3"]
pub type REventSel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_event_en` reader - r_event_en"]
pub type REventEnR = crate::FieldReader;
#[doc = "Field `r_event_en` writer - r_event_en"]
pub type REventEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - r_event_sel_0"]
    #[inline(always)]
    pub fn r_event_sel_0(&self) -> REventSel0R {
        REventSel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - r_event_sel_1"]
    #[inline(always)]
    pub fn r_event_sel_1(&self) -> REventSel1R {
        REventSel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - r_event_sel_2"]
    #[inline(always)]
    pub fn r_event_sel_2(&self) -> REventSel2R {
        REventSel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - r_event_sel_3"]
    #[inline(always)]
    pub fn r_event_sel_3(&self) -> REventSel3R {
        REventSel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - r_event_en"]
    #[inline(always)]
    pub fn r_event_en(&self) -> REventEnR {
        REventEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - r_event_sel_0"]
    #[inline(always)]
    pub fn r_event_sel_0(&mut self) -> REventSel0W<'_, RegEventCfgSpec> {
        REventSel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - r_event_sel_1"]
    #[inline(always)]
    pub fn r_event_sel_1(&mut self) -> REventSel1W<'_, RegEventCfgSpec> {
        REventSel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - r_event_sel_2"]
    #[inline(always)]
    pub fn r_event_sel_2(&mut self) -> REventSel2W<'_, RegEventCfgSpec> {
        REventSel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - r_event_sel_3"]
    #[inline(always)]
    pub fn r_event_sel_3(&mut self) -> REventSel3W<'_, RegEventCfgSpec> {
        REventSel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - r_event_en"]
    #[inline(always)]
    pub fn r_event_en(&mut self) -> REventEnW<'_, RegEventCfgSpec> {
        REventEnW::new(self, 16)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_event_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_event_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegEventCfgSpec;
impl crate::RegisterSpec for RegEventCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_event_cfg::R`](R) reader structure"]
impl crate::Readable for RegEventCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_event_cfg::W`](W) writer structure"]
impl crate::Writable for RegEventCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EVENT_CFG to value 0"]
impl crate::Resettable for RegEventCfgSpec {}
