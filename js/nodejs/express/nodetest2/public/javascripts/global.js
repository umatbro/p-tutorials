// userlist data array
let userListData = [];

// DOM ready
$(document).ready(() => {
    populateTable();

    // username link click
    $('#userList').find('table tbody').on('click', 'td a.linkshowuser', showUserInfo);

    // add user button click
    $('#btnAddUser').on('click', addUser);
});

function populateTable() {
    // empty content string
    let tableContent = '';

    $.getJSON('/users/userlist', data => {
        userListData = data;

        // for each item in JSON add table row and cells
        $.each(data, (index, user) => {
            tableContent +=
                `<tr>
                    <td><a href="#" class="linkshowuser" rel="${user.username}">${user.username}</a></td>
                    <td>${user.email}</td>
                    <td><a href="#" class="linkdeleteuser" rel="${user._id}">delete</a></td>
                </tr>`
        });
        // Inject the whole content string into our existing HTML table
        $('#userList').find('table tbody').html(tableContent);
    });
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

function addUser(event) {
    event.preventDefault();

    let errorCount = 0;
    $('#addUser').find('input').each((index, val) => {
        let field = val;
        console.log(field);
        if (field.value === '') errorCount++;
    });

    if (errorCount === 0) {
        // if no errors: compose user object
        let newUser = {
            username: $('#inputUserName').val(),
            email: $('#inputUserEmail').val(),
            fullname: $('#inputUserFullName').val(),
            age: $('#inputUserAge').val(),
            location: $('#inputUserLocation').val(),
            gender: $('#inputUserGender').val()
        };

        // use AJAX to post the object to our add user service
        $.ajax({
            type: 'POST',
            data: newUser,
            url: '/users/adduser',
            dataType: 'JSON'
        }).done(response => {
            if (response.msg === '') {
                // clear form inputs
                $('#addUser').find('fieldset input').val('');
                // update table
                populateTable();
            } else {  // if something goes wrong - alert the message
                alert(`Error: ${response.msg}`);
            }
        });
    } else {
        alert(`Please fill in all fields`);
        return false;
    }
}

