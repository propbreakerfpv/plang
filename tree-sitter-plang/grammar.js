module.exports = grammar({
  name: 'plang',

  rules: {
    source_file: $ => repeat($._definition),
    _definition: $ => choice(
      $.ident,
      $.function_definition
    ),
    function_definition: $ => seq(
      'fn',
      $.ident,
      $.paramiter_list,
      $._type,
      $.block
    ),
    ident: $ => /[a-z]+/,
    paramiter_list: $ => seq(
      '(',
      repeat(
        seq(
          $.ident,
          $,type
        )
      ),
      ')'
    ),
    // this is not done yet
    _type: $ => seq(
      ':',
      $.ident,
    ),
    block: $ => seq(
      '{',
      repeat($._definition),
      '}'
    )
  }
});
