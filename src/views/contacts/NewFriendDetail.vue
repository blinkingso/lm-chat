<template>
    <div>New Friend Detail</div>
</template>

<script>
export default {
    props: {
        contact_detail: {
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
    display: block;
    width: 100%;
    height: auto;
}

td {
    color: #ccc;
}

tr {
    padding: 0 10px;
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
    display: block;
    text-align: right !important;
}

.td-header-right img {
    margin-right: 20px;
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

.message {
    margin-top: 10px;
    width: 120px;
    height: 40px;
    border-radius: 10px;
    border: none;
    background-color: limegreen;
}
</style>