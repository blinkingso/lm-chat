<template>
    <div class="detail">
        <div>
            <table>
                <tr>
                    <td class="td-header">
                        {{ user.nick_name }}
                        <img src="../../assets/female.png" />
                    </td>
                    <td rowspan="2" class="td-header-right"><img :src="user.avatar" /></td>
                </tr>
                <tr>
                    <td>{{ user.nick_name }}</td>
                </tr>
            </table>
        </div>
        <span class="line"></span>
        <table>
            <tbody>
                <tr>
                    <td>Alias</td>
                    <td class="td-text">{{ user.nick_name }}
                    </td>
                </tr>
                <tr>
                    <td>Region</td>
                    <td class="td-text">China mainland</td>
                </tr>
                <tr>
                    <td>LmChat ID</td>
                    <td class="td-text">{{ user.name }}</td>
                </tr>
                <tr>
                    <td>From</td>
                    <td class="td-text">Mobile Contacts</td>
                </tr>
                <tr>
                    <td>Privacy</td>
                    <td class="td-text"></td>
                </tr>
            </tbody>
        </table>
        <span class="line"></span>
        <div>
            <table>
                <tr>
                    <td>Moments</td>
                    <td></td>
                </tr>
                <tr></tr>
                <tr col="2">
                    <td class="td-button">
                        <button class="message">Message</button>
                    </td>
                </tr>
            </table>
        </div>
    </div>
</template>

<script>
export default {
    props: {
        user: {
            type: Object
        }
    },
    data() {
        return {}
    },
    created() {
    }
}
</script>

<style scoped>
.detail {
    bottom: 0;
    position: fixed;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: #FFF;
    display: block;
    text-align: center;
}

table {
    margin-left: 10%;
    border: none;
    display: table;
    width: 50%;
}

td {
    color: #ccc;
}

tr {
    padding: 0;
    width: 100%;
    height: 40px;
    text-align: left;
    vertical-align: middle;
}

.td-header {
    font-size: 1.5em;
    color: #000;
}

.td-header img {
    width: 16px;
    height: 16px;
}

.td-header-right {
    text-align: right !important;
}

.td-header-right img {
    width: 120px;
    height: 120px;
    margin-left: 30%;
}

.td-text {
    padding-left: 20px;
    color: #000;
}

.line {
    margin-left: 10%;
    width: 80%;
    height: 0;
    border-top: 1px solid #ccc;
    display: block;
    margin-bottom: 10px;
}

.mm-container {
    text-align: center;
}

.moments {
    color: #ccc;
    left: 10%;
    position: absolute;
}

.td-button {
    text-align: center;
}

.message {
    width: 120px;
    height: 40px;
    border-radius: 10px;
    border: none;
    background-color: limegreen;
}
</style>