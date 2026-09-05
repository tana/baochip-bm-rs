#[doc = "Register `SFR_BLKT0` reader"]
pub type R = crate::R<SfrBlkt0Spec>;
#[doc = "Register `SFR_BLKT0` writer"]
pub type W = crate::W<SfrBlkt0Spec>;
#[doc = "Field `sfr_blkt0` reader - sfr_blkt0 read/write control register"]
pub type SfrBlkt0R = crate::FieldReader;
#[doc = "Field `sfr_blkt0` writer - sfr_blkt0 read/write control register"]
pub type SfrBlkt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_blkt0 read/write control register"]
    #[inline(always)]
    pub fn sfr_blkt0(&self) -> SfrBlkt0R {
        SfrBlkt0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_blkt0 read/write control register"]
    #[inline(always)]
    pub fn sfr_blkt0(&mut self) -> SfrBlkt0W<'_, SfrBlkt0Spec> {
        SfrBlkt0W::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L216 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L216>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_blkt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_blkt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrBlkt0Spec;
impl crate::RegisterSpec for SfrBlkt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_blkt0::R`](R) reader structure"]
impl crate::Readable for SfrBlkt0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_blkt0::W`](W) writer structure"]
impl crate::Writable for SfrBlkt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_BLKT0 to value 0"]
impl crate::Resettable for SfrBlkt0Spec {}
