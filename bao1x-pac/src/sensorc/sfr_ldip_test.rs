#[doc = "Register `SFR_LDIP_TEST` reader"]
pub type R = crate::R<SfrLdipTestSpec>;
#[doc = "Register `SFR_LDIP_TEST` writer"]
pub type W = crate::W<SfrLdipTestSpec>;
#[doc = "Field `ldtst` reader - ldtst read/write control register"]
pub type LdtstR = crate::FieldReader;
#[doc = "Field `ldtst` writer - ldtst read/write control register"]
pub type LdtstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ldtst read/write control register"]
    #[inline(always)]
    pub fn ldtst(&self) -> LdtstR {
        LdtstR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ldtst read/write control register"]
    #[inline(always)]
    pub fn ldtst(&mut self) -> LdtstW<'_, SfrLdipTestSpec> {
        LdtstW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L77>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldip_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldip_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrLdipTestSpec;
impl crate::RegisterSpec for SfrLdipTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ldip_test::R`](R) reader structure"]
impl crate::Readable for SfrLdipTestSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ldip_test::W`](W) writer structure"]
impl crate::Writable for SfrLdipTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_LDIP_TEST to value 0"]
impl crate::Resettable for SfrLdipTestSpec {}
