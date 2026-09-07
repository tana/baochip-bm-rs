#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `trng_done` reader - Write a ``1`` to enable the ``trng_done`` Event"]
pub type TrngDoneR = crate::BitReader;
#[doc = "Field `trng_done` writer - Write a ``1`` to enable the ``trng_done`` Event"]
pub type TrngDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_done` reader - Write a ``1`` to enable the ``aes_done`` Event"]
pub type AesDoneR = crate::BitReader;
#[doc = "Field `aes_done` writer - Write a ``1`` to enable the ``aes_done`` Event"]
pub type AesDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_done` reader - Write a ``1`` to enable the ``pke_done`` Event"]
pub type PkeDoneR = crate::BitReader;
#[doc = "Field `pke_done` writer - Write a ``1`` to enable the ``pke_done`` Event"]
pub type PkeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_done` reader - Write a ``1`` to enable the ``hash_done`` Event"]
pub type HashDoneR = crate::BitReader;
#[doc = "Field `hash_done` writer - Write a ``1`` to enable the ``hash_done`` Event"]
pub type HashDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alu_done` reader - Write a ``1`` to enable the ``alu_done`` Event"]
pub type AluDoneR = crate::BitReader;
#[doc = "Field `alu_done` writer - Write a ``1`` to enable the ``alu_done`` Event"]
pub type AluDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_ichdone` reader - Write a ``1`` to enable the ``sdma_ichdone`` Event"]
pub type SdmaIchdoneR = crate::BitReader;
#[doc = "Field `sdma_ichdone` writer - Write a ``1`` to enable the ``sdma_ichdone`` Event"]
pub type SdmaIchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_schdone` reader - Write a ``1`` to enable the ``sdma_schdone`` Event"]
pub type SdmaSchdoneR = crate::BitReader;
#[doc = "Field `sdma_schdone` writer - Write a ``1`` to enable the ``sdma_schdone`` Event"]
pub type SdmaSchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_xchdone` reader - Write a ``1`` to enable the ``sdma_xchdone`` Event"]
pub type SdmaXchdoneR = crate::BitReader;
#[doc = "Field `sdma_xchdone` writer - Write a ``1`` to enable the ``sdma_xchdone`` Event"]
pub type SdmaXchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s8` reader - Write a ``1`` to enable the ``nc_b3s8`` Event"]
pub type NcB3s8R = crate::BitReader;
#[doc = "Field `nc_b3s8` writer - Write a ``1`` to enable the ``nc_b3s8`` Event"]
pub type NcB3s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s9` reader - Write a ``1`` to enable the ``nc_b3s9`` Event"]
pub type NcB3s9R = crate::BitReader;
#[doc = "Field `nc_b3s9` writer - Write a ``1`` to enable the ``nc_b3s9`` Event"]
pub type NcB3s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s10` reader - Write a ``1`` to enable the ``nc_b3s10`` Event"]
pub type NcB3s10R = crate::BitReader;
#[doc = "Field `nc_b3s10` writer - Write a ``1`` to enable the ``nc_b3s10`` Event"]
pub type NcB3s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s11` reader - Write a ``1`` to enable the ``nc_b3s11`` Event"]
pub type NcB3s11R = crate::BitReader;
#[doc = "Field `nc_b3s11` writer - Write a ``1`` to enable the ``nc_b3s11`` Event"]
pub type NcB3s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s12` reader - Write a ``1`` to enable the ``nc_b3s12`` Event"]
pub type NcB3s12R = crate::BitReader;
#[doc = "Field `nc_b3s12` writer - Write a ``1`` to enable the ``nc_b3s12`` Event"]
pub type NcB3s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s13` reader - Write a ``1`` to enable the ``nc_b3s13`` Event"]
pub type NcB3s13R = crate::BitReader;
#[doc = "Field `nc_b3s13` writer - Write a ``1`` to enable the ``nc_b3s13`` Event"]
pub type NcB3s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s14` reader - Write a ``1`` to enable the ``nc_b3s14`` Event"]
pub type NcB3s14R = crate::BitReader;
#[doc = "Field `nc_b3s14` writer - Write a ``1`` to enable the ``nc_b3s14`` Event"]
pub type NcB3s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s15` reader - Write a ``1`` to enable the ``nc_b3s15`` Event"]
pub type NcB3s15R = crate::BitReader;
#[doc = "Field `nc_b3s15` writer - Write a ``1`` to enable the ``nc_b3s15`` Event"]
pub type NcB3s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``trng_done`` Event"]
    #[inline(always)]
    pub fn trng_done(&self) -> TrngDoneR {
        TrngDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``aes_done`` Event"]
    #[inline(always)]
    pub fn aes_done(&self) -> AesDoneR {
        AesDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``pke_done`` Event"]
    #[inline(always)]
    pub fn pke_done(&self) -> PkeDoneR {
        PkeDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``hash_done`` Event"]
    #[inline(always)]
    pub fn hash_done(&self) -> HashDoneR {
        HashDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``alu_done`` Event"]
    #[inline(always)]
    pub fn alu_done(&self) -> AluDoneR {
        AluDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``sdma_ichdone`` Event"]
    #[inline(always)]
    pub fn sdma_ichdone(&self) -> SdmaIchdoneR {
        SdmaIchdoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``sdma_schdone`` Event"]
    #[inline(always)]
    pub fn sdma_schdone(&self) -> SdmaSchdoneR {
        SdmaSchdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``sdma_xchdone`` Event"]
    #[inline(always)]
    pub fn sdma_xchdone(&self) -> SdmaXchdoneR {
        SdmaXchdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b3s8`` Event"]
    #[inline(always)]
    pub fn nc_b3s8(&self) -> NcB3s8R {
        NcB3s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b3s9`` Event"]
    #[inline(always)]
    pub fn nc_b3s9(&self) -> NcB3s9R {
        NcB3s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b3s10`` Event"]
    #[inline(always)]
    pub fn nc_b3s10(&self) -> NcB3s10R {
        NcB3s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b3s11`` Event"]
    #[inline(always)]
    pub fn nc_b3s11(&self) -> NcB3s11R {
        NcB3s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b3s12`` Event"]
    #[inline(always)]
    pub fn nc_b3s12(&self) -> NcB3s12R {
        NcB3s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b3s13`` Event"]
    #[inline(always)]
    pub fn nc_b3s13(&self) -> NcB3s13R {
        NcB3s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b3s14`` Event"]
    #[inline(always)]
    pub fn nc_b3s14(&self) -> NcB3s14R {
        NcB3s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b3s15`` Event"]
    #[inline(always)]
    pub fn nc_b3s15(&self) -> NcB3s15R {
        NcB3s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``trng_done`` Event"]
    #[inline(always)]
    pub fn trng_done(&mut self) -> TrngDoneW<'_, EvEnableSpec> {
        TrngDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``aes_done`` Event"]
    #[inline(always)]
    pub fn aes_done(&mut self) -> AesDoneW<'_, EvEnableSpec> {
        AesDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``pke_done`` Event"]
    #[inline(always)]
    pub fn pke_done(&mut self) -> PkeDoneW<'_, EvEnableSpec> {
        PkeDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``hash_done`` Event"]
    #[inline(always)]
    pub fn hash_done(&mut self) -> HashDoneW<'_, EvEnableSpec> {
        HashDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``alu_done`` Event"]
    #[inline(always)]
    pub fn alu_done(&mut self) -> AluDoneW<'_, EvEnableSpec> {
        AluDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``sdma_ichdone`` Event"]
    #[inline(always)]
    pub fn sdma_ichdone(&mut self) -> SdmaIchdoneW<'_, EvEnableSpec> {
        SdmaIchdoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``sdma_schdone`` Event"]
    #[inline(always)]
    pub fn sdma_schdone(&mut self) -> SdmaSchdoneW<'_, EvEnableSpec> {
        SdmaSchdoneW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``sdma_xchdone`` Event"]
    #[inline(always)]
    pub fn sdma_xchdone(&mut self) -> SdmaXchdoneW<'_, EvEnableSpec> {
        SdmaXchdoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b3s8`` Event"]
    #[inline(always)]
    pub fn nc_b3s8(&mut self) -> NcB3s8W<'_, EvEnableSpec> {
        NcB3s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b3s9`` Event"]
    #[inline(always)]
    pub fn nc_b3s9(&mut self) -> NcB3s9W<'_, EvEnableSpec> {
        NcB3s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b3s10`` Event"]
    #[inline(always)]
    pub fn nc_b3s10(&mut self) -> NcB3s10W<'_, EvEnableSpec> {
        NcB3s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b3s11`` Event"]
    #[inline(always)]
    pub fn nc_b3s11(&mut self) -> NcB3s11W<'_, EvEnableSpec> {
        NcB3s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b3s12`` Event"]
    #[inline(always)]
    pub fn nc_b3s12(&mut self) -> NcB3s12W<'_, EvEnableSpec> {
        NcB3s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b3s13`` Event"]
    #[inline(always)]
    pub fn nc_b3s13(&mut self) -> NcB3s13W<'_, EvEnableSpec> {
        NcB3s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b3s14`` Event"]
    #[inline(always)]
    pub fn nc_b3s14(&mut self) -> NcB3s14W<'_, EvEnableSpec> {
        NcB3s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b3s15`` Event"]
    #[inline(always)]
    pub fn nc_b3s15(&mut self) -> NcB3s15W<'_, EvEnableSpec> {
        NcB3s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
