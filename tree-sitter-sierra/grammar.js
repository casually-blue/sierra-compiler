module.exports = grammar({
  name: 'sierra',

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat($.syntax_unit),
    syntax_unit: $ => choice(
      $.use_statement, 
      $.program_statement,
      $.type_definition,
      $.implementation_block,
    ),

    use_statement: $ => seq('use', $.qualified_identifier, ';'),

    program_statement: $ => seq('program', $.identifier, ';'),

    type_definition: $ => seq('type', $.identifier, '=', $.expression, ';'),

    implementation_block: $ => seq('implementation', $.identifier, 
      repeat($.function_definition), 'end'),

    function_definition: $ => seq(
      optional($.visibility_specifier), 'function', $.identifier, optional($.parameter_list),
      optional(seq(':', $.qualified_identifier)), $.block
    ),

    parameter_list: $ => 
      seq('(', $.parameter_list_item_recursive, ')'),
    parameter_list_item_recursive: $ => 
      choice($.parameter_specifier, seq($.parameter_specifier, ',', $.parameter_list_item_recursive)),

    parameter_specifier: $ => 
      seq(optional($.mutability_specifier), choice(
        $.identifier, ':', $.identifier),
        'self'),

    block: $ => seq('begin', 
      repeat($.statement), optional($.expression),
    'end'),

    statement: $ => seq ($.expression, optional(seq(':=', $.expression)), ';'),

    expression: $ => choice(
      $.number,
      $.identifier,
      prec.left(1, seq($.expression, '..', $.expression)),
      seq($.identifier, '[', $.identifier, ']', 'of', $.identifier),
    ),

    visibility_specifier: $ => choice('public', 'private'),
    mutability_specifier: $ => choice('const', 'mutable'),

    qualified_identifier: $ => choice($.identifier, seq($.identifier, '.', $.qualified_identifier)),
    identifier: $ => /[a-zA-Z_][a-z-A-Z0-9_]*/,
    number: $ => /[0-9][0-9_]*/,
  }
});

