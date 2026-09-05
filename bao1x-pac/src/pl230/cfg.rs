#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `MASTER_ENABLE` writer - MASTER_ENABLE"]
pub type MasterEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL_PROT_CTRL` reader - Set AHB-Lite configuration"]
pub type ChnlProtCtrlR = crate::FieldReader;
impl R {
    #[doc = "Bits 5:7 - Set AHB-Lite configuration"]
    #[inline(always)]
    pub fn chnl_prot_ctrl(&self) -> ChnlProtCtrlR {
        ChnlProtCtrlR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MASTER_ENABLE"]
    #[inline(always)]
    pub fn master_enable(&mut self) -> MasterEnableW<'_, CfgSpec> {
        MasterEnableW::new(self, 0)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
