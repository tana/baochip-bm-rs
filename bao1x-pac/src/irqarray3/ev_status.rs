#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `trng_done` reader - Level of the ``trng_done`` event"]
pub type TrngDoneR = crate::BitReader;
#[doc = "Field `trng_done` writer - Level of the ``trng_done`` event"]
pub type TrngDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_done` reader - Level of the ``aes_done`` event"]
pub type AesDoneR = crate::BitReader;
#[doc = "Field `aes_done` writer - Level of the ``aes_done`` event"]
pub type AesDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_done` reader - Level of the ``pke_done`` event"]
pub type PkeDoneR = crate::BitReader;
#[doc = "Field `pke_done` writer - Level of the ``pke_done`` event"]
pub type PkeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_done` reader - Level of the ``hash_done`` event"]
pub type HashDoneR = crate::BitReader;
#[doc = "Field `hash_done` writer - Level of the ``hash_done`` event"]
pub type HashDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alu_done` reader - Level of the ``alu_done`` event"]
pub type AluDoneR = crate::BitReader;
#[doc = "Field `alu_done` writer - Level of the ``alu_done`` event"]
pub type AluDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_ichdone` reader - Level of the ``sdma_ichdone`` event"]
pub type SdmaIchdoneR = crate::BitReader;
#[doc = "Field `sdma_ichdone` writer - Level of the ``sdma_ichdone`` event"]
pub type SdmaIchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_schdone` reader - Level of the ``sdma_schdone`` event"]
pub type SdmaSchdoneR = crate::BitReader;
#[doc = "Field `sdma_schdone` writer - Level of the ``sdma_schdone`` event"]
pub type SdmaSchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_xchdone` reader - Level of the ``sdma_xchdone`` event"]
pub type SdmaXchdoneR = crate::BitReader;
#[doc = "Field `sdma_xchdone` writer - Level of the ``sdma_xchdone`` event"]
pub type SdmaXchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s8` reader - Level of the ``nc_b3s8`` event"]
pub type NcB3s8R = crate::BitReader;
#[doc = "Field `nc_b3s8` writer - Level of the ``nc_b3s8`` event"]
pub type NcB3s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s9` reader - Level of the ``nc_b3s9`` event"]
pub type NcB3s9R = crate::BitReader;
#[doc = "Field `nc_b3s9` writer - Level of the ``nc_b3s9`` event"]
pub type NcB3s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s10` reader - Level of the ``nc_b3s10`` event"]
pub type NcB3s10R = crate::BitReader;
#[doc = "Field `nc_b3s10` writer - Level of the ``nc_b3s10`` event"]
pub type NcB3s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s11` reader - Level of the ``nc_b3s11`` event"]
pub type NcB3s11R = crate::BitReader;
#[doc = "Field `nc_b3s11` writer - Level of the ``nc_b3s11`` event"]
pub type NcB3s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s12` reader - Level of the ``nc_b3s12`` event"]
pub type NcB3s12R = crate::BitReader;
#[doc = "Field `nc_b3s12` writer - Level of the ``nc_b3s12`` event"]
pub type NcB3s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s13` reader - Level of the ``nc_b3s13`` event"]
pub type NcB3s13R = crate::BitReader;
#[doc = "Field `nc_b3s13` writer - Level of the ``nc_b3s13`` event"]
pub type NcB3s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s14` reader - Level of the ``nc_b3s14`` event"]
pub type NcB3s14R = crate::BitReader;
#[doc = "Field `nc_b3s14` writer - Level of the ``nc_b3s14`` event"]
pub type NcB3s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s15` reader - Level of the ``nc_b3s15`` event"]
pub type NcB3s15R = crate::BitReader;
#[doc = "Field `nc_b3s15` writer - Level of the ``nc_b3s15`` event"]
pub type NcB3s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``trng_done`` event"]
    #[inline(always)]
    pub fn trng_done(&self) -> TrngDoneR {
        TrngDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``aes_done`` event"]
    #[inline(always)]
    pub fn aes_done(&self) -> AesDoneR {
        AesDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``pke_done`` event"]
    #[inline(always)]
    pub fn pke_done(&self) -> PkeDoneR {
        PkeDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``hash_done`` event"]
    #[inline(always)]
    pub fn hash_done(&self) -> HashDoneR {
        HashDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``alu_done`` event"]
    #[inline(always)]
    pub fn alu_done(&self) -> AluDoneR {
        AluDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``sdma_ichdone`` event"]
    #[inline(always)]
    pub fn sdma_ichdone(&self) -> SdmaIchdoneR {
        SdmaIchdoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``sdma_schdone`` event"]
    #[inline(always)]
    pub fn sdma_schdone(&self) -> SdmaSchdoneR {
        SdmaSchdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``sdma_xchdone`` event"]
    #[inline(always)]
    pub fn sdma_xchdone(&self) -> SdmaXchdoneR {
        SdmaXchdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b3s8`` event"]
    #[inline(always)]
    pub fn nc_b3s8(&self) -> NcB3s8R {
        NcB3s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b3s9`` event"]
    #[inline(always)]
    pub fn nc_b3s9(&self) -> NcB3s9R {
        NcB3s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b3s10`` event"]
    #[inline(always)]
    pub fn nc_b3s10(&self) -> NcB3s10R {
        NcB3s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b3s11`` event"]
    #[inline(always)]
    pub fn nc_b3s11(&self) -> NcB3s11R {
        NcB3s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b3s12`` event"]
    #[inline(always)]
    pub fn nc_b3s12(&self) -> NcB3s12R {
        NcB3s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b3s13`` event"]
    #[inline(always)]
    pub fn nc_b3s13(&self) -> NcB3s13R {
        NcB3s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b3s14`` event"]
    #[inline(always)]
    pub fn nc_b3s14(&self) -> NcB3s14R {
        NcB3s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b3s15`` event"]
    #[inline(always)]
    pub fn nc_b3s15(&self) -> NcB3s15R {
        NcB3s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``trng_done`` event"]
    #[inline(always)]
    pub fn trng_done(&mut self) -> TrngDoneW<'_, EvStatusSpec> {
        TrngDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``aes_done`` event"]
    #[inline(always)]
    pub fn aes_done(&mut self) -> AesDoneW<'_, EvStatusSpec> {
        AesDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``pke_done`` event"]
    #[inline(always)]
    pub fn pke_done(&mut self) -> PkeDoneW<'_, EvStatusSpec> {
        PkeDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``hash_done`` event"]
    #[inline(always)]
    pub fn hash_done(&mut self) -> HashDoneW<'_, EvStatusSpec> {
        HashDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``alu_done`` event"]
    #[inline(always)]
    pub fn alu_done(&mut self) -> AluDoneW<'_, EvStatusSpec> {
        AluDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``sdma_ichdone`` event"]
    #[inline(always)]
    pub fn sdma_ichdone(&mut self) -> SdmaIchdoneW<'_, EvStatusSpec> {
        SdmaIchdoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``sdma_schdone`` event"]
    #[inline(always)]
    pub fn sdma_schdone(&mut self) -> SdmaSchdoneW<'_, EvStatusSpec> {
        SdmaSchdoneW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``sdma_xchdone`` event"]
    #[inline(always)]
    pub fn sdma_xchdone(&mut self) -> SdmaXchdoneW<'_, EvStatusSpec> {
        SdmaXchdoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b3s8`` event"]
    #[inline(always)]
    pub fn nc_b3s8(&mut self) -> NcB3s8W<'_, EvStatusSpec> {
        NcB3s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b3s9`` event"]
    #[inline(always)]
    pub fn nc_b3s9(&mut self) -> NcB3s9W<'_, EvStatusSpec> {
        NcB3s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b3s10`` event"]
    #[inline(always)]
    pub fn nc_b3s10(&mut self) -> NcB3s10W<'_, EvStatusSpec> {
        NcB3s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b3s11`` event"]
    #[inline(always)]
    pub fn nc_b3s11(&mut self) -> NcB3s11W<'_, EvStatusSpec> {
        NcB3s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b3s12`` event"]
    #[inline(always)]
    pub fn nc_b3s12(&mut self) -> NcB3s12W<'_, EvStatusSpec> {
        NcB3s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b3s13`` event"]
    #[inline(always)]
    pub fn nc_b3s13(&mut self) -> NcB3s13W<'_, EvStatusSpec> {
        NcB3s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b3s14`` event"]
    #[inline(always)]
    pub fn nc_b3s14(&mut self) -> NcB3s14W<'_, EvStatusSpec> {
        NcB3s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b3s15`` event"]
    #[inline(always)]
    pub fn nc_b3s15(&mut self) -> NcB3s15W<'_, EvStatusSpec> {
        NcB3s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStatusSpec;
impl crate::RegisterSpec for EvStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EvStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_status::W`](W) writer structure"]
impl crate::Writable for EvStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_STATUS to value 0"]
impl crate::Resettable for EvStatusSpec {}
