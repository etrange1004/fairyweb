pub const WRITE_FORM_SCRIPT: &str = "
function write_quest_submit() {
    if ( document.write_quest.title.value.trim() == \"\" || document.write_quest.title.value == null ) {
        alert(\"제목을 입력해주세요! ^0^n\");
        document.write_quest.title.focus();
        return;
    }
    if ( document.write_quest.name.value.trim() == \"\" || document.write_quest.name.value == null ) {
        alert(\"이름을 입력해주세요! ^0^n\");
        document.write_quest.name.focus();
        return;
    }
    if ( document.write_quest.password.value.trim() == \"\" || document.write_quest.password.value == null ) {
        alert(\"비밀번호를 입력해주세요! ^0^n\");
        document.write_quest.password.focus();
        return;
    }
    if ( document.write_quest.content.value.trim() == \"\" || document.write_quest.content.value == null ) {
        alert(\"내용을 입력해주세요! ^0^n\");
        document.write_quest.content.focus();
        return;
    }
    document.write_quest.submit();    
}";
pub const EDIT_FORM_SCRIPT: &str = "
function edit_quest_submit() {
    if ( document.edit_quest.title.value.trim() == \"\" || document.edit_quest.title.value == null ) {
        alert(\"제목을 입력해주세요! ^0^n\");
        document.edit_quest.title.focus();
        return;
    }
    if ( document.edit_quest.password.value.trim() == \"\" || document.edit_quest.password.value == null ) {
        alert(\"비밀번호를 입력해주세요! ^0^n\");
        document.edit_quest.password.focus();
        return;
    }
    if ( document.edit_quest.content.value.trim() == \"\" || document.edit_quest.content.value == null ) {
        alert(\"내용을 입력해주세요! ^0^n\");
        document.edit_quest.content.focus();
        return;
    }
    document.edit_quest.submit();    
}";
pub const COMMENT_FORM_SCRIPT: &str = "
function write_comment_submit() {
    if ( document.write_comment.content.value.trim() == \"\" || document.write_comment.content.value == null ) {
        alert(\"내용을 입력해주세요! ^0^n\");
        document.write_comment.content.focus();
        return;
    }
    document.write_comment.submit();    
}";
pub const SIGNIN_FORM_SCRIPT: &str = "
function signin_submit() {
    if ( document.signin.email.value.trim() == \"\" || document.signin.email.value == null ) {
        alert(\"이메일 아이디를 입력해주세요! ^0^n\");
        document.signin.email.focus();
        return;
    }
    if ( document.signin.name.value.trim() == \"\" || document.signin.name.value == null ) {
        alert(\"이름을 입력해주세요! ^0^n\");
        document.signin.name.focus();
        return;
    }
    if ( document.signin.password.value.trim() == \"\" || document.signin.password.value == null ) {
        alert(\"비밀번호를 입력해주세요! ^0^n\");
        document.signin.password.focus();
        return;
    }
    if ( document.signin.bio.value.trim() == \"\" || document.signin.bio.value == null ) {
        alert(\"자기소개 내용을 입력해주세요! ^0^n\");
        document.signin.bio.focus();
        return;
    }
    document.signin.submit();    
}";
pub const EDIT_USER_FORM_SCRIPT: &str = "
function edituser_submit() {
    if ( document.edituser.email.value.trim() == \"\" || document.edituser.email.value == null ) {
        alert(\"이메일 아이디를 입력해주세요! ^0^n\");
        document.edituser.email.focus();
        return;
    }
    if ( document.edituser.name.value.trim() == \"\" || document.edituser.name.value == null ) {
        alert(\"이름을 입력해주세요! ^0^n\");
        document.edituser.name.focus();
        return;
    }
    if ( document.edituser.password.value.trim() == \"\" || document.edituser.password.value == null ) {
        alert(\"비밀번호를 입력해주세요! ^0^n\");
        document.edituser.password.focus();
        return;
    }
    if ( document.edituser.bio.value.trim() == \"\" || document.edituser.bio.value == null ) {
        alert(\"자기소개 내용을 입력해주세요! ^0^n\");
        document.edituser.bio.focus();
        return;
    }
    document.edituser.submit();    
}";