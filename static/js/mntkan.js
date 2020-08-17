var mntkanApp = {}

function insertCard(params) {
  fetch('/cards.json', {
    method: 'post',
    headers: new Headers({'Content-type': 'application/json'}),
    body: JSON.stringify(params)
  }).then(function(response) {
    return response.json()
  }).then(function(data) {
    console.log('created card:', data)
  })
}

function updateCard(params) {
  fetch('/cards/'+params.id+'.json', {
    method: 'put',
    headers: new Headers({'Content-type': 'application/json'}),
    body: JSON.stringify(params)
  }).then(function(response) {
    return response.json()
  }).then(function(data) {
    console.log('updated card:', data)
  })
}

function findCard(id) {
  var lists = mntkanApp.$data.lists
  
  for (var i=0; i<lists.length; i++) {
    var cards = lists[i].cards    
    for (var j=0; j<cards.length; j++) {
      var card = cards[j]
      
      if (id == card.id) {
        return card
      }
    }
  }
  
  return null
}

function boot() {
  mntkanApp = new Vue({
    el: '#app',
    data: {
      lists: [
        {
          title: "List 1",
          collapsed: false,
          cards: [
            {
              title: "Loading"
            },
          ]
        },
      ],
      newCard: {
        list_id: "todo",
        title: "",
        body: ""
      },
      activeCard: {
        id: "",
        list_id: "todo",
        title: "",
        body: ""
      }
    },
    methods: {
      insertCardSubmit: function(t) {
        console.log("insertCardSubmit",t)
        t.preventDefault()

        insertCard(this.$data.newCard)
        return false
      },
      updateCardSubmit: function(t) {
        console.log("updateCardSubmit",t)
        t.preventDefault()

        updateCard(this.$data.activeCard)
        return false
      },
      handleCardDrop: function(t) {
        console.log("handleCardDrop",t)
        var card_id = t.item.id
        var id_from = t.from.id
        var id_to = t.to.id
        console.log(card_id, ":", id_from, "->", id_to)

        var card = findCard(card_id)
        if (card) {
          card.list_id = id_to
          updateCard(card)
        }
      }
    }
  })

  fetch('/cards.json')
    .then(response => response.json())
    .then(data => {
      console.log(data)
      mntkanApp.$data.lists = []
      var lists = [
        {
          title: "To Do",
          id: "todo",
          collapsed: false,
          cards: []
        },
        {
          title: "Waiting",
          id: "blocked",
          collapsed: false,
          cards: []
        },
        {
          title: "Done",
          id: "done",
          collapsed: false,
          cards: []
        }
      ]

      for (var i=0; i<data.length; i++) {
        var card = data[i]       
        for (var j=0; j<lists.length; j++) {
          var list = lists[j]
          
          if (list.id == card.list_id) {
            list.cards.push(card)
          }
        }
      }

      list.collapsed = false

      /*lists.sort((a,b) => {
        if (a.title<b.title) return -1;
        if (a.title>b.title) return 1;
        return 0;
        })*/

      mntkanApp.$data.lists = lists
    })
}

