use async_trait::async_trait;
use wechaty_puppet::*;

#[derive(Debug)]
pub struct PuppetMock {}

#[allow(dead_code)]
#[async_trait]
impl PuppetImpl for PuppetMock {
    async fn contact_self_name_set(&mut self, name: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_self_qr_code(&mut self) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn contact_self_signature_set(&mut self, signature: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn tag_contact_add(&mut self, tag_id: String, contact_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn tag_contact_remove(&mut self, tag_id: String, contact_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn tag_contact_delete(&mut self, tag_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn tag_contact_list(&mut self, contact_id: String) -> Result<Vec<String>, PuppetError> {
        unimplemented!()
    }

    async fn tag_list(&mut self) -> Result<Vec<String>, PuppetError> {
        unimplemented!()
    }

    async fn contact_alias(&mut self, contact_id: String) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn contact_alias_set(&mut self, contact_id: String, alias: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_avatar(&mut self, contact_id: String) -> Result<FileBox, PuppetError> {
        unimplemented!()
    }

    async fn contact_avatar_set(&mut self, contact_id: String, file: FileBox) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_phone_set(&mut self, contact_id: String, phone_list: Vec<String>) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_corporation_remark_set(
        &mut self,
        contact_id: String,
        corporation_remark: Option<String>,
    ) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_description_set(
        &mut self,
        contact_id: String,
        description: Option<String>,
    ) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn contact_list(&mut self) -> Result<Vec<String>, PuppetError> {
        unimplemented!()
    }

    async fn contact_raw_payload(&mut self, contact_id: String) -> Result<ContactPayload, PuppetError> {
        unimplemented!()
    }

    async fn message_contact(&mut self, message_id: String) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn message_file(&mut self, message_id: String) -> Result<FileBox, PuppetError> {
        unimplemented!()
    }

    async fn message_image(&mut self, message_id: String, image_type: ImageType) -> Result<FileBox, PuppetError> {
        unimplemented!()
    }

    async fn message_mini_program(&mut self, message_id: String) -> Result<MiniProgramPayload, PuppetError> {
        unimplemented!()
    }

    async fn message_url(&mut self, message_id: String) -> Result<UrlLinkPayload, PuppetError> {
        unimplemented!()
    }

    async fn message_send_contact(
        &mut self,
        conversation_id: String,
        contact_id: String,
    ) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn message_send_file(
        &mut self,
        conversation_id: String,
        file: FileBox,
    ) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn message_send_mini_program(
        &mut self,
        conversation_id: String,
        mini_program_payload: MiniProgramPayload,
    ) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn message_send_text(
        &mut self,
        conversation_id: String,
        text: String,
        mention_id_list: Vec<String>,
    ) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn message_send_url(
        &mut self,
        conversation_id: String,
        url_link_payload: UrlLinkPayload,
    ) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn message_raw_payload(&mut self, message_id: String) -> Result<MessagePayload, PuppetError> {
        unimplemented!()
    }

    async fn friendship_accept(&mut self, friendship_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn friendship_add(&mut self, contact_id: String, hello: Option<String>) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn friendship_search_phone(&mut self, phone: String) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn friendship_search_weixin(&mut self, weixin: String) -> Result<Option<String>, PuppetError> {
        unimplemented!()
    }

    async fn friendship_raw_payload(&mut self, friendship_id: String) -> Result<FriendshipPayload, PuppetError> {
        unimplemented!()
    }

    async fn room_invitation_accept(&mut self, room_invitation_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_invitation_raw_payload(
        &mut self,
        room_invitation_id: String,
    ) -> Result<RoomInvitationPayload, PuppetError> {
        unimplemented!()
    }

    async fn room_add(&mut self, room_id: String, contact_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_avatar(&mut self, room_id: String) -> Result<FileBox, PuppetError> {
        unimplemented!()
    }

    async fn room_create(
        &mut self,
        contact_id_list: Vec<String>,
        topic: Option<String>,
    ) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn room_del(&mut self, room_id: String, contact_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_qr_code(&mut self, room_id: String) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn room_quit(&mut self, room_id: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_topic(&mut self, room_id: String) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn room_topic_set(&mut self, room_id: String, topic: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_list(&mut self) -> Result<Vec<String>, PuppetError> {
        unimplemented!()
    }

    async fn room_raw_payload(&mut self, room_id: String) -> Result<RoomPayload, PuppetError> {
        unimplemented!()
    }

    async fn room_announce(&mut self, room_id: String) -> Result<String, PuppetError> {
        unimplemented!()
    }

    async fn room_announce_set(&mut self, room_id: String, text: String) -> Result<(), PuppetError> {
        unimplemented!()
    }

    async fn room_member_list(&mut self, room_id: String) -> Result<Vec<String>, PuppetError> {
        unimplemented!()
    }

    async fn room_member_raw_payload(
        &mut self,
        room_id: String,
        contact_id: String,
    ) -> Result<RoomMemberPayload, PuppetError> {
        unimplemented!()
    }
}
