#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `trng_done_dupe` reader - Level of the ``trng_done_dupe`` event"]
pub type TrngDoneDupeR = crate::BitReader;
#[doc = "Field `trng_done_dupe` writer - Level of the ``trng_done_dupe`` event"]
pub type TrngDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_done_dupe` reader - Level of the ``aes_done_dupe`` event"]
pub type AesDoneDupeR = crate::BitReader;
#[doc = "Field `aes_done_dupe` writer - Level of the ``aes_done_dupe`` event"]
pub type AesDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_done_dupe` reader - Level of the ``pke_done_dupe`` event"]
pub type PkeDoneDupeR = crate::BitReader;
#[doc = "Field `pke_done_dupe` writer - Level of the ``pke_done_dupe`` event"]
pub type PkeDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_done_dupe` reader - Level of the ``hash_done_dupe`` event"]
pub type HashDoneDupeR = crate::BitReader;
#[doc = "Field `hash_done_dupe` writer - Level of the ``hash_done_dupe`` event"]
pub type HashDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alu_done_dupe` reader - Level of the ``alu_done_dupe`` event"]
pub type AluDoneDupeR = crate::BitReader;
#[doc = "Field `alu_done_dupe` writer - Level of the ``alu_done_dupe`` event"]
pub type AluDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_ichdone_dupe` reader - Level of the ``sdma_ichdone_dupe`` event"]
pub type SdmaIchdoneDupeR = crate::BitReader;
#[doc = "Field `sdma_ichdone_dupe` writer - Level of the ``sdma_ichdone_dupe`` event"]
pub type SdmaIchdoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_schdone_dupe` reader - Level of the ``sdma_schdone_dupe`` event"]
pub type SdmaSchdoneDupeR = crate::BitReader;
#[doc = "Field `sdma_schdone_dupe` writer - Level of the ``sdma_schdone_dupe`` event"]
pub type SdmaSchdoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_xchdone_dupe` reader - Level of the ``sdma_xchdone_dupe`` event"]
pub type SdmaXchdoneDupeR = crate::BitReader;
#[doc = "Field `sdma_xchdone_dupe` writer - Level of the ``sdma_xchdone_dupe`` event"]
pub type SdmaXchdoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s8` reader - Level of the ``nc_b4s8`` event"]
pub type NcB4s8R = crate::BitReader;
#[doc = "Field `nc_b4s8` writer - Level of the ``nc_b4s8`` event"]
pub type NcB4s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s9` reader - Level of the ``nc_b4s9`` event"]
pub type NcB4s9R = crate::BitReader;
#[doc = "Field `nc_b4s9` writer - Level of the ``nc_b4s9`` event"]
pub type NcB4s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s10` reader - Level of the ``nc_b4s10`` event"]
pub type NcB4s10R = crate::BitReader;
#[doc = "Field `nc_b4s10` writer - Level of the ``nc_b4s10`` event"]
pub type NcB4s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s11` reader - Level of the ``nc_b4s11`` event"]
pub type NcB4s11R = crate::BitReader;
#[doc = "Field `nc_b4s11` writer - Level of the ``nc_b4s11`` event"]
pub type NcB4s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s12` reader - Level of the ``nc_b4s12`` event"]
pub type NcB4s12R = crate::BitReader;
#[doc = "Field `nc_b4s12` writer - Level of the ``nc_b4s12`` event"]
pub type NcB4s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s13` reader - Level of the ``nc_b4s13`` event"]
pub type NcB4s13R = crate::BitReader;
#[doc = "Field `nc_b4s13` writer - Level of the ``nc_b4s13`` event"]
pub type NcB4s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s14` reader - Level of the ``nc_b4s14`` event"]
pub type NcB4s14R = crate::BitReader;
#[doc = "Field `nc_b4s14` writer - Level of the ``nc_b4s14`` event"]
pub type NcB4s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b4s15` reader - Level of the ``nc_b4s15`` event"]
pub type NcB4s15R = crate::BitReader;
#[doc = "Field `nc_b4s15` writer - Level of the ``nc_b4s15`` event"]
pub type NcB4s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``trng_done_dupe`` event"]
    #[inline(always)]
    pub fn trng_done_dupe(&self) -> TrngDoneDupeR {
        TrngDoneDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``aes_done_dupe`` event"]
    #[inline(always)]
    pub fn aes_done_dupe(&self) -> AesDoneDupeR {
        AesDoneDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``pke_done_dupe`` event"]
    #[inline(always)]
    pub fn pke_done_dupe(&self) -> PkeDoneDupeR {
        PkeDoneDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``hash_done_dupe`` event"]
    #[inline(always)]
    pub fn hash_done_dupe(&self) -> HashDoneDupeR {
        HashDoneDupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``alu_done_dupe`` event"]
    #[inline(always)]
    pub fn alu_done_dupe(&self) -> AluDoneDupeR {
        AluDoneDupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``sdma_ichdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_ichdone_dupe(&self) -> SdmaIchdoneDupeR {
        SdmaIchdoneDupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``sdma_schdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_schdone_dupe(&self) -> SdmaSchdoneDupeR {
        SdmaSchdoneDupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``sdma_xchdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_xchdone_dupe(&self) -> SdmaXchdoneDupeR {
        SdmaXchdoneDupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b4s8`` event"]
    #[inline(always)]
    pub fn nc_b4s8(&self) -> NcB4s8R {
        NcB4s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b4s9`` event"]
    #[inline(always)]
    pub fn nc_b4s9(&self) -> NcB4s9R {
        NcB4s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b4s10`` event"]
    #[inline(always)]
    pub fn nc_b4s10(&self) -> NcB4s10R {
        NcB4s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b4s11`` event"]
    #[inline(always)]
    pub fn nc_b4s11(&self) -> NcB4s11R {
        NcB4s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b4s12`` event"]
    #[inline(always)]
    pub fn nc_b4s12(&self) -> NcB4s12R {
        NcB4s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b4s13`` event"]
    #[inline(always)]
    pub fn nc_b4s13(&self) -> NcB4s13R {
        NcB4s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b4s14`` event"]
    #[inline(always)]
    pub fn nc_b4s14(&self) -> NcB4s14R {
        NcB4s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b4s15`` event"]
    #[inline(always)]
    pub fn nc_b4s15(&self) -> NcB4s15R {
        NcB4s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``trng_done_dupe`` event"]
    #[inline(always)]
    pub fn trng_done_dupe(&mut self) -> TrngDoneDupeW<'_, EvStatusSpec> {
        TrngDoneDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``aes_done_dupe`` event"]
    #[inline(always)]
    pub fn aes_done_dupe(&mut self) -> AesDoneDupeW<'_, EvStatusSpec> {
        AesDoneDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``pke_done_dupe`` event"]
    #[inline(always)]
    pub fn pke_done_dupe(&mut self) -> PkeDoneDupeW<'_, EvStatusSpec> {
        PkeDoneDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``hash_done_dupe`` event"]
    #[inline(always)]
    pub fn hash_done_dupe(&mut self) -> HashDoneDupeW<'_, EvStatusSpec> {
        HashDoneDupeW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``alu_done_dupe`` event"]
    #[inline(always)]
    pub fn alu_done_dupe(&mut self) -> AluDoneDupeW<'_, EvStatusSpec> {
        AluDoneDupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``sdma_ichdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_ichdone_dupe(&mut self) -> SdmaIchdoneDupeW<'_, EvStatusSpec> {
        SdmaIchdoneDupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``sdma_schdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_schdone_dupe(&mut self) -> SdmaSchdoneDupeW<'_, EvStatusSpec> {
        SdmaSchdoneDupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``sdma_xchdone_dupe`` event"]
    #[inline(always)]
    pub fn sdma_xchdone_dupe(&mut self) -> SdmaXchdoneDupeW<'_, EvStatusSpec> {
        SdmaXchdoneDupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b4s8`` event"]
    #[inline(always)]
    pub fn nc_b4s8(&mut self) -> NcB4s8W<'_, EvStatusSpec> {
        NcB4s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b4s9`` event"]
    #[inline(always)]
    pub fn nc_b4s9(&mut self) -> NcB4s9W<'_, EvStatusSpec> {
        NcB4s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b4s10`` event"]
    #[inline(always)]
    pub fn nc_b4s10(&mut self) -> NcB4s10W<'_, EvStatusSpec> {
        NcB4s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b4s11`` event"]
    #[inline(always)]
    pub fn nc_b4s11(&mut self) -> NcB4s11W<'_, EvStatusSpec> {
        NcB4s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b4s12`` event"]
    #[inline(always)]
    pub fn nc_b4s12(&mut self) -> NcB4s12W<'_, EvStatusSpec> {
        NcB4s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b4s13`` event"]
    #[inline(always)]
    pub fn nc_b4s13(&mut self) -> NcB4s13W<'_, EvStatusSpec> {
        NcB4s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b4s14`` event"]
    #[inline(always)]
    pub fn nc_b4s14(&mut self) -> NcB4s14W<'_, EvStatusSpec> {
        NcB4s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b4s15`` event"]
    #[inline(always)]
    pub fn nc_b4s15(&mut self) -> NcB4s15W<'_, EvStatusSpec> {
        NcB4s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b4s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
