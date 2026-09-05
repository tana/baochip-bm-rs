#[doc = "Register `SFR_CRFUNC` reader"]
pub type R = crate::R<SfrCrfuncSpec>;
#[doc = "Register `SFR_CRFUNC` writer"]
pub type W = crate::W<SfrCrfuncSpec>;
#[doc = "Field `cr_func` reader - cr_func read/write control register"]
pub type CrFuncR = crate::FieldReader;
#[doc = "Field `cr_func` writer - cr_func read/write control register"]
pub type CrFuncW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_pcoreir` reader - cr_pcoreir read/write control register"]
pub type CrPcoreirR = crate::FieldReader;
#[doc = "Field `cr_pcoreir` writer - cr_pcoreir read/write control register"]
pub type CrPcoreirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cr_func read/write control register"]
    #[inline(always)]
    pub fn cr_func(&self) -> CrFuncR {
        CrFuncR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - cr_pcoreir read/write control register"]
    #[inline(always)]
    pub fn cr_pcoreir(&self) -> CrPcoreirR {
        CrPcoreirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cr_func read/write control register"]
    #[inline(always)]
    pub fn cr_func(&mut self) -> CrFuncW<'_, SfrCrfuncSpec> {
        CrFuncW::new(self, 0)
    }
    #[doc = "Bits 8:15 - cr_pcoreir read/write control register"]
    #[inline(always)]
    pub fn cr_pcoreir(&mut self) -> CrPcoreirW<'_, SfrCrfuncSpec> {
        CrPcoreirW::new(self, 8)
    }
}
#[doc = "See `pke.sv#L294 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L294>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrfuncSpec;
impl crate::RegisterSpec for SfrCrfuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_crfunc::R`](R) reader structure"]
impl crate::Readable for SfrCrfuncSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_crfunc::W`](W) writer structure"]
impl crate::Writable for SfrCrfuncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CRFUNC to value 0"]
impl crate::Resettable for SfrCrfuncSpec {}
