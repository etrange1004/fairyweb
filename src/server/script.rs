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