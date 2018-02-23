// userlist data array
let userListData = [];

// DOM ready
$(document).ready(() => {
    populateTable();
});

function populateTable() {
    // empty content string
    let tableContent = '';

    $.getJSON('/users/userlist', data => {
        userListData = data;

        // for each item in JSON add table row and cells
        $.each(data, (index, user) => {
            tableContent =
                `<tr>
                    <td><a href="#" class="linkshowuser" rel="${user.username}">${user.username}</a></td>
                    <td>${user.email}</td>
                    <td><a href="#" class="linkdeleteuser" rel="${user._id}">delete</a></td>
                    </td>
                </tr>`
        });
        // Inject the whole content string into our existing HTML table
        $('#userList').find('table tbody').html(tableContent);
    });

    // username link click
    $('#userList').find('table tbody').on('click', 'td a.linkshowuser', showUserInfo)
}

function showUserInfo(event) {
    event.preventDefault();
    let thisUserName = $(this).attr('rel');
    // get index of object based on id value
    let arrayPosition = userListData.map((arrayItem) => arrayItem.username).indexOf(thisUserName);

    let thisUserObject = userListData[arrayPosition];
    // populate infobox
    $('#userInfoName').text(thisUserObject.fullname);
    $('#userInfoAge').text(thisUserObject.age);
    $('#userInfoGender').text(thisUserObject.gender);
    $('#userInfoLocation').text(thisUserObject.location);
}