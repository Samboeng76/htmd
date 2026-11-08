git Test migration inventory
============================

This is a planning table for all 221 named integration tests in `tests/`. The
`Faithful round trip` column concerns **Faithful-mode** HTML → Markdown → HTML
checks only. A candidate may need a compact fixture; this table does not claim
that a new tree comparison has already passed. Keep Pure-mode assertions in
mixed tests.

| Round trip            | Meaning                                                                                          |
| --------------------- | ------------------------------------------------------------------------------------------------ |
| Yes                   | Compare the rendered HTML with the input body using the proposed tree rules.                     |
| Expected              | Use an explicit expected HTML result for a documented transformation, parser repair, or wrapper. |
| Configured            | Use the test's htmd options or the matching pulldown-cmark extension.                            |
| Configured + expected | Both configuration and explicit expected HTML are needed.                                        |
| No                    | The existing test has no suitable Faithful round-trip claim. Keep its current purpose.           |

`Keep Markdown = Yes` means its Markdown output assertion protects syntax,
escaping, fallback, options, or an intentional loss. `No` means the test
currently checks rendered HTML or its simple Markdown assertion could be
replaced once the standardized comparison covers the same behavior. `N/A` means
the test has no Markdown-output assertion; keep its other checks.

`Existing round trip` records current input-derived HTML → Markdown → HTML
checks, including calls through local helpers and explicit rendering with
pulldown-cmark. `Faithful` means every Faithful input case in the test is
rendered back; `Partial` names the Faithful cases that are not. `Both` means the
test renders both modes, `Pure only` means it renders only Pure mode, and `No`
means it has no input-derived round trip. `N/A` marks unit tests and
documentation examples outside the conversion pipeline. Rendering a fixed
Markdown literal alone does not count. Existing checks may compare HTML strings
or only selected HTML features; this column does not claim that they use the
proposed standardized comparison.

Root-level inline content may gain a `<p>` on rendering. Pretty-printed input
may contain whitespace text nodes that htmd does not preserve. Those cases need
a compact fixture or explicit expected HTML; they do not justify a global rule
that ignores wrappers or whitespace. The known `<br><br>` root failure remains a
failure of preservation, even though an existing test records the HTML it
currently renders.

Integration tests
-----------------

### [tests/basic\_tests.rs](tests/basic_tests.rs) (85)

| Test                                                                                         | Faithful round trip   | Keep Markdown | Existing round trip                 |
| -------------------------------------------------------------------------------------------- | --------------------- | ------------- | ----------------------------------- |
| [`links_with_spaces`](tests/basic_tests.rs#L16)                                              | Expected              | Yes           | No                                  |
| [`referenced_links_with_title`](tests/basic_tests.rs#L27)                                    | Configured            | Yes           | No                                  |
| [`consecutive_referenced_links_with_title`](tests/basic_tests.rs#L47)                        | Configured            | Yes           | No                                  |
| [`images`](tests/basic_tests.rs#L73)                                                         | Expected              | Yes           | No                                  |
| [`images_with_spaces_in_url`](tests/basic_tests.rs#L87)                                      | Expected              | Yes           | No                                  |
| [`image_title_stays_outside_an_angle_bracket_destination`](tests/basic_tests.rs#L98)         | No                    | Yes           | No                                  |
| [`headings`](tests/basic_tests.rs#L111)                                                      | Yes                   | Yes           | No                                  |
| [`paragraphs`](tests/basic_tests.rs#L128)                                                    | Yes                   | Yes           | No                                  |
| [`quotes`](tests/basic_tests.rs#L140)                                                        | Expected              | Yes           | No                                  |
| [`br`](tests/basic_tests.rs#L148)                                                            | Yes                   | Yes           | No                                  |
| [`hr`](tests/basic_tests.rs#L155)                                                            | Expected              | Yes           | No                                  |
| [`strong_italic`](tests/basic_tests.rs#L161)                                                 | Expected              | Yes           | No                                  |
| [`faithful_mode_keeps_adjacent_emphasis_apart`](tests/basic_tests.rs#L178)                   | Yes                   | Yes           | No                                  |
| [`faithful_mode_keeps_adjacent_code_spans_apart`](tests/basic_tests.rs#L197)                 | Yes                   | Yes           | No                                  |
| [`faithful_mode_keeps_adjacent_untranslated_elements_apart`](tests/basic_tests.rs#L207)      | Yes                   | Yes           | No                                  |
| [`pure_mode_merges_adjacent_inline_elements`](tests/basic_tests.rs#L224)                     | No                    | Yes           | No                                  |
| [`adjacent_math_spans_stay_math`](tests/basic_tests.rs#L236)                                 | Configured            | Yes           | Both                                |
| [`blank_emphasis_leaves_its_whitespace_behind`](tests/basic_tests.rs#L270)                   | No                    | Yes           | No                                  |
| [`italic_inside_word`](tests/basic_tests.rs#L281)                                            | Expected              | Yes           | No                                  |
| [`italic_with_br`](tests/basic_tests.rs#L287)                                                | Expected              | Yes           | No                                  |
| [`inline_raw_html_escaping`](tests/basic_tests.rs#L296)                                      | Expected              | Yes           | No                                  |
| [`multiline_raw_html_escaping`](tests/basic_tests.rs#L305)                                   | Expected              | Yes           | No                                  |
| [`inline_email_autolink_escaping`](tests/basic_tests.rs#L328)                                | Yes                   | Yes           | Faithful                            |
| [`a_sequence_finished_by_a_sibling_stays_text`](tests/basic_tests.rs#L347)                   | No                    | Yes           | Pure only                           |
| [`html_escaping`](tests/basic_tests.rs#L383)                                                 | Yes                   | Yes           | No                                  |
| [`faithful_mode_inline`](tests/basic_tests.rs#L413)                                          | Expected              | Yes           | No                                  |
| [`faithful_mode_hr`](tests/basic_tests.rs#L433)                                              | Yes                   | Yes           | No                                  |
| [`faithful_mode_blockquote`](tests/basic_tests.rs#L441)                                      | Expected              | Yes           | No                                  |
| [`faithful_mode_h1`](tests/basic_tests.rs#L461)                                              | Yes                   | Yes           | No                                  |
| [`faithful_mode_p`](tests/basic_tests.rs#L469)                                               | Yes                   | Yes           | No                                  |
| [`faithful_mode_ol1`](tests/basic_tests.rs#L477)                                             | Yes                   | Yes           | No                                  |
| [`faithful_mode_ol2`](tests/basic_tests.rs#L498)                                             | Yes                   | Yes           | No                                  |
| [`faithful_mode_comment`](tests/basic_tests.rs#L515)                                         | Yes                   | Yes           | No                                  |
| [`faithful_mode_html`](tests/basic_tests.rs#L523)                                            | Expected              | Yes           | No                                  |
| [`faithful_mode_table`](tests/basic_tests.rs#L546)                                           | Expected              | Yes           | No                                  |
| [`faithful_mode_serializes_a_table_with_a_caption`](tests/basic_tests.rs#L589)               | Yes                   | Yes           | Faithful                            |
| [`pure_mode_writes_a_caption_as_a_paragraph`](tests/basic_tests.rs#L611)                     | No                    | Yes           | No                                  |
| [`pure_mode_separates_a_caption_from_the_table_it_sits_above`](tests/basic_tests.rs#L628)    | No                    | Yes           | Pure only                           |
| [`faithful_mode_nested_inline_html`](tests/basic_tests.rs#L655)                              | Yes                   | Yes           | No                                  |
| [`spaces_check`](tests/basic_tests.rs#L663)                                                  | Expected              | Yes           | No                                  |
| [`consecutive_blocks`](tests/basic_tests.rs#L672)                                            | Yes                   | No            | No                                  |
| [`raw_text`](tests/basic_tests.rs#L686)                                                      | Expected              | Yes           | No                                  |
| [`nested_divs`](tests/basic_tests.rs#L692)                                                   | No                    | Yes           | No                                  |
| [`with_head`](tests/basic_tests.rs#L706)                                                     | No                    | Yes           | No                                  |
| [`with_custom_rules`](tests/basic_tests.rs#L726)                                             | No                    | Yes           | No                                  |
| [`with_custom_rules_and_fallback`](tests/basic_tests.rs#L738)                                | No                    | Yes           | No                                  |
| [`upper_case_tags`](tests/basic_tests.rs#L766)                                               | Expected              | Yes           | No                                  |
| [`html_entities`](tests/basic_tests.rs#L775)                                                 | Yes                   | Yes           | Partial (first two Faithful inputs) |
| [`scripting_option`](tests/basic_tests.rs#L793)                                              | No                    | Yes           | No                                  |
| [`multithreading`](tests/basic_tests.rs#L811)                                                | No                    | Yes           | No                                  |
| [`unterminated_html`](tests/basic_tests.rs#L847)                                             | Expected              | Yes           | No                                  |
| [`misnested_formatting_does_not_duplicate_or_lose_text`](tests/basic_tests.rs#L853)          | No                    | Yes           | No                                  |
| [`math`](tests/basic_tests.rs#L860)                                                          | Configured            | Yes           | No                                  |
| [`empty_math_span`](tests/basic_tests.rs#L894)                                               | Configured            | Yes           | Faithful                            |
| [`math_span_whitespace`](tests/basic_tests.rs#L909)                                          | Configured + expected | Yes           | Partial (trimmed inline math)       |
| [`unwritable_math_span`](tests/basic_tests.rs#L934)                                          | Configured            | Yes           | Faithful                            |
| [`span_not_of_the_math_form`](tests/basic_tests.rs#L963)                                     | Configured            | Yes           | Faithful                            |
| [`math_span_in_a_table_cell`](tests/basic_tests.rs#L978)                                     | Configured + expected | Yes           | Faithful                            |
| [`dollar_sign_in_text`](tests/basic_tests.rs#L996)                                           | Configured            | Yes           | Faithful                            |
| [`document_whitespace`](tests/basic_tests.rs#L1003)                                          | Yes                   | Yes           | No                                  |
| [`multibyte_ordered_list_escape_half`](tests/basic_tests.rs#L1021)                           | Yes                   | Yes           | No                                  |
| [`multibyte_ordered_list_escape_accented`](tests/basic_tests.rs#L1027)                       | Yes                   | Yes           | No                                  |
| [`multibyte_ordered_list_escape_trademark`](tests/basic_tests.rs#L1034)                      | Yes                   | Yes           | No                                  |
| [`ascii_ordered_list_escape`](tests/basic_tests.rs#L1041)                                    | Yes                   | Yes           | No                                  |
| [`multibyte_no_dot`](tests/basic_tests.rs#L1047)                                             | Yes                   | Yes           | No                                  |
| [`cjk_before_ordered_list`](tests/basic_tests.rs#L1054)                                      | Yes                   | Yes           | No                                  |
| [`multibyte_atx_heading_escape`](tests/basic_tests.rs#L1061)                                 | Yes                   | Yes           | No                                  |
| [`multibyte_atx_heading_escape_umlaut`](tests/basic_tests.rs#L1067)                          | Yes                   | Yes           | No                                  |
| [`a_paragraph_of_leaf_block_syntax_stays_a_paragraph`](tests/basic_tests.rs#L1101)           | Yes                   | Yes           | Faithful                            |
| [`an_atx_marker_alone_in_a_paragraph_is_escaped`](tests/basic_tests.rs#L1119)                | Yes                   | Yes           | Faithful                            |
| [`a_bullet_marker_closed_by_a_line_ending_is_escaped`](tests/basic_tests.rs#L1142)           | Expected              | Yes           | Faithful                            |
| [`an_atx_marker_closed_by_a_line_ending_is_escaped`](tests/basic_tests.rs#L1151)             | Expected              | Yes           | Faithful                            |
| [`an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped`](tests/basic_tests.rs#L1160) | Expected              | Yes           | Faithful                            |
| [`a_marker_behind_leading_whitespace_is_escaped`](tests/basic_tests.rs#L1170)                | Expected              | Yes           | Faithful                            |
| [`a_marker_which_starts_no_line_is_not_escaped`](tests/basic_tests.rs#L1184)                 | Yes                   | Yes           | Faithful                            |
| [`a_run_of_ordered_item_delimiters_opens_no_list`](tests/basic_tests.rs#L1197)               | Yes                   | Yes           | Faithful                            |
| [`a_paragraph_of_unicode_numerics_needs_no_escape`](tests/basic_tests.rs#L1208)              | Yes                   | Yes           | Faithful                            |
| [`a_literal_cdata_section_is_lost_unescaped`](tests/basic_tests.rs#L1224)                    | Expected              | No            | Faithful                            |
| [`round_trip_in_a_block_context`](tests/basic_tests.rs#L1255)                                | Expected              | No            | Faithful                            |
| [`round_trip_of_a_leaf_block_an_html_block_would_dissolve`](tests/basic_tests.rs#L1294)      | Yes                   | No            | Faithful                            |
| [`round_trip_of_a_setext_heading_of_leaf_block_syntax`](tests/basic_tests.rs#L1345)          | Configured            | No            | Faithful                            |
| [`round_trip_in_an_inline_context`](tests/basic_tests.rs#L1359)                              | Expected              | No            | Faithful                            |
| [`round_trip_of_a_walked_raw_inline`](tests/basic_tests.rs#L1417)                            | Yes                   | No            | Faithful                            |
| [`round_trip_losses_of_a_walked_raw_inline`](tests/basic_tests.rs#L1454)                     | Expected              | No            | Faithful                            |
| [`append_content_when_the_body_is_empty`](tests/basic_tests.rs#L1524)                        | No                    | Yes           | No                                  |

### [tests/code\_tests.rs](tests/code_tests.rs) (20)

| Test                                                                                                 | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`code_blocks`](tests/code_tests.rs#L29)                                                             | Expected            | Yes           | No                  |
| [`code_blocks_with_lang_class`](tests/code_tests.rs#L40)                                             | Expected            | Yes           | No                  |
| [`faithful_mode_preserves_an_empty_language_class`](tests/code_tests.rs#L51)                         | Yes                 | Yes           | No                  |
| [`code_blocks_decode_html_entities`](tests/code_tests.rs#L58)                                        | Expected            | Yes           | No                  |
| [`code_blocks_with_lang_class_on_pre_tag`](tests/code_tests.rs#L71)                                  | No                  | Yes           | No                  |
| [`span_subtree_conversion_preserves_ancestor_preformatted_context`](tests/code_tests.rs#L82)         | No                  | Yes           | No                  |
| [`delegated_unhandled_subtree_preserves_ancestor_preformatted_context`](tests/code_tests.rs#L93)     | No                  | Yes           | No                  |
| [`faithful_mode_pre`](tests/code_tests.rs#L113)                                                      | Yes                 | Yes           | No                  |
| [`faithful_mode_code_block1`](tests/code_tests.rs#L121)                                              | Yes                 | Yes           | No                  |
| [`faithful_mode_code_block2`](tests/code_tests.rs#L129)                                              | Yes                 | Yes           | No                  |
| [`inline_code_made_only_of_backticks_uses_a_non_colliding_delimiter`](tests/code_tests.rs#L140)      | No                  | Yes           | No                  |
| [`inline_code_ending_in_a_backtick_keeps_the_backtick_inside_the_span`](tests/code_tests.rs#L147)    | No                  | Yes           | No                  |
| [`preformatted_inline_code_preserves_boundary_spaces`](tests/code_tests.rs#L154)                     | No                  | Yes           | No                  |
| [`pure_mode_writes_a_code_span_line_ending_as_the_space_commonmark_reads`](tests/code_tests.rs#L173) | No                  | Yes           | No                  |
| [`fenced_code_uses_a_fence_longer_than_any_run_in_its_content`](tests/code_tests.rs#L185)            | No                  | Yes           | No                  |
| [`tilde_fenced_code_uses_a_fence_longer_than_any_run_in_its_content`](tests/code_tests.rs#L193)      | No                  | Yes           | No                  |
| [`processing_instruction_nodes_are_ignored`](tests/code_tests.rs#L209)                               | No                  | Yes           | No                  |
| [`code_block_with_spans_ending_in_newline`](tests/code_tests.rs#L220)                                | No                  | Yes           | No                  |
| [`code_block_with_blank_lines_and_indentation_in_spans`](tests/code_tests.rs#L232)                   | No                  | Yes           | No                  |
| [`span_fallback_in_pre_preserves_newlines`](tests/code_tests.rs#L245)                                | No                  | Yes           | No                  |

### [tests/context\_tests.rs](tests/context_tests.rs) (26)

| Test                                                                                       | Faithful round trip   | Keep Markdown | Existing round trip |
| ------------------------------------------------------------------------------------------ | --------------------- | ------------- | ------------------- |
| [`html_at_the_document_root_is_a_block`](tests/context_tests.rs#L11)                       | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33)       | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55)    | Expected              | Yes           | No                  |
| [`raw_text_at_the_document_root_is_left_alone`](tests/context_tests.rs#L89)                | Expected              | Yes           | No                  |
| [`html_in_a_paragraph_is_a_raw_inline`](tests/context_tests.rs#L94)                        | Yes                   | Yes           | No                  |
| [`html_in_a_heading_is_a_raw_inline`](tests/context_tests.rs#L111)                         | Yes                   | Yes           | No                  |
| [`html_in_a_blockquote_is_a_block`](tests/context_tests.rs#L124)                           | Expected              | Yes           | No                  |
| [`html_in_a_list_item_is_a_block`](tests/context_tests.rs#L140)                            | Expected              | Yes           | No                  |
| [`html_in_a_table_cell_is_a_raw_inline`](tests/context_tests.rs#L160)                      | Configured            | Yes           | No                  |
| [`a_cell_attribute_writes_the_whole_table_as_html`](tests/context_tests.rs#L194)           | Yes                   | Yes           | No                  |
| [`html_in_an_inline_element_is_a_raw_inline`](tests/context_tests.rs#L204)                 | Yes                   | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222)                              | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260)          | Expected              | Yes           | No                  |
| [`an_rcdata_element_is_translated`](tests/context_tests.rs#L295)                           | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320)      | Expected              | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347)   | Configured + expected | Yes           | No                  |
| [`a_raw_inline_encodes_the_pipes_of_its_attributes`](tests/context_tests.rs#L391)          | Configured            | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417)                      | Expected              | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450)                          | Configured + expected | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494)                             | Expected              | Yes           | No                  |
| [`a_cdata_section_and_a_processing_instruction_are_comments`](tests/context_tests.rs#L527) | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546)    | Expected              | Yes           | No                  |
| [`a_serialized_element_follows_its_context`](tests/context_tests.rs#L575)                  | Yes                   | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594)         | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628)                           | Configured + expected | Yes           | No                  |
| [`an_unwritable_math_span_strips_its_comments`](tests/context_tests.rs#L677)               | Configured + expected | Yes           | No                  |

### [tests/event\_tests.rs](tests/event_tests.rs) (6)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`test_element_events_skip_on_handler_fallback`](tests/event_tests.rs#L13)         | No                  | Yes           | No                  |
| [`test_passive_tag_filtering`](tests/event_tests.rs#L77)                           | No                  | N/A           | No                  |
| [`test_doc_lifecycle_events_with_nesting`](tests/event_tests.rs#L128)              | No                  | N/A           | No                  |
| [`test_span_events_dispatched_in_pure_mode`](tests/event_tests.rs#L194)            | No                  | Yes           | No                  |
| [`test_unhandled_tag_events_dispatched_in_pure_mode`](tests/event_tests.rs#L248)   | No                  | Yes           | No                  |
| [`test_span_and_unhandled_tag_events_in_faithful_mode`](tests/event_tests.rs#L302) | No                  | N/A           | No                  |

### [tests/link\_tests.rs](tests/link_tests.rs) (26)

| Test                                                                                  | Faithful round trip   | Keep Markdown | Existing round trip                                  |
| ------------------------------------------------------------------------------------- | --------------------- | ------------- | ---------------------------------------------------- |
| [`links`](tests/link_tests.rs#L12)                                                    | Expected              | Yes           | No                                                   |
| [`links_with_spaces_in_destination_and_title`](tests/link_tests.rs#L24)               | Expected              | Yes           | No                                                   |
| [`links_with_spaces_around_text`](tests/link_tests.rs#L33)                            | Expected              | Yes           | No                                                   |
| [`links_inlined_prefer_autolinks`](tests/link_tests.rs#L45)                           | Configured + expected | Yes           | No                                                   |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74)              | Expected              | Yes           | No                                                   |
| [`nested_convert_with_referenced_links`](tests/link_tests.rs#L109)                    | No                    | Yes           | No                                                   |
| [`discard_links_in_table_falling_back_to_raw_html`](tests/link_tests.rs#L164)         | Configured + expected | Yes           | No                                                   |
| [`a_link_destination_escapes_its_backslashes`](tests/link_tests.rs#L212)              | Expected              | Yes           | Partial (image and other link destinations)          |
| [`a_link_title_escapes_its_backslashes`](tests/link_tests.rs#L243)                    | Expected              | Yes           | No                                                   |
| [`an_image_alt_escapes_its_markdown_specials`](tests/link_tests.rs#L268)              | Expected              | Yes           | Partial (unmatched bracket and other specials)       |
| [`an_image_alt_escapes_its_raw_html`](tests/link_tests.rs#L296)                       | Expected              | Yes           | Partial (non-tag angle bracket)                      |
| [`a_bracketed_link_destination_escapes_its_angle_brackets`](tests/link_tests.rs#L320) | Expected              | Yes           | Partial (angle bracket opening the destination)      |
| [`an_autolink_spells_its_destination_without_escapes`](tests/link_tests.rs#L342)      | Configured + expected | Yes           | Partial (backslash URL)                              |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366)                       | Configured + expected | Yes           | No                                                   |
| [`a_referenced_link_destination_brackets_its_spaces`](tests/link_tests.rs#L404)       | Configured + expected | Yes           | No                                                   |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413)                         | Configured            | Yes           | Partial (other Faithful inputs and referenced style) |
| [`a_link_escapes_its_ampersands`](tests/link_tests.rs#L454)                           | Configured + expected | Yes           | Partial (referenced style)                           |
| [`link_text_escapes_its_math_delimiters`](tests/link_tests.rs#L483)                   | Configured            | Yes           | Faithful                                             |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503)   | Expected              | Yes           | Partial (empty-source image and referenced style)    |
| [`link_text_encodes_its_line_endings`](tests/link_tests.rs#L547)                      | Expected              | Yes           | Partial (referenced style)                           |
| [`an_image_is_inline_in_the_referenced_style`](tests/link_tests.rs#L576)              | Configured + expected | Yes           | No                                                   |
| [`a_link_keeps_the_whitespace_around_its_text`](tests/link_tests.rs#L586)             | Expected              | Yes           | Partial (other spacing cases and referenced style)   |
| [`an_image_without_a_source_is_written_as_html`](tests/link_tests.rs#L617)            | Yes                   | Yes           | Faithful                                             |
| [`an_image_without_alt_text_is_written_as_html`](tests/link_tests.rs#L633)            | Expected              | Yes           | Faithful                                             |
| [`an_image_with_an_empty_title_is_written_as_html`](tests/link_tests.rs#L659)         | Yes                   | Yes           | Faithful                                             |
| [`a_link_escapes_an_email_autolink`](tests/link_tests.rs#L675)                        | Expected              | Yes           | Faithful                                             |

### [tests/list\_tests.rs](tests/list_tests.rs) (8)

| Test                                                                             | Faithful round trip   | Keep Markdown | Existing round trip |
| -------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`unordered_lists`](tests/list_tests.rs#L6)                                      | Yes                   | Yes           | No                  |
| [`unordered_lists_custom_bullet_spacing`](tests/list_tests.rs#L21)               | Configured            | Yes           | No                  |
| [`ordered_lists`](tests/list_tests.rs#L38)                                       | Yes                   | Yes           | No                  |
| [`ordered_lists_custom_bullet_spacing`](tests/list_tests.rs#L53)                 | Configured            | Yes           | No                  |
| [`ordered_lists_start_with_zero_or_negative`](tests/list_tests.rs#L70)           | Configured + expected | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95)           | Configured            | Yes           | No                  |
| [`ordered_list_alignment_across_a_digit_boundary`](tests/list_tests.rs#L130)     | Yes                   | Yes           | No                  |
| [`ordered_list_alignment_yields_to_the_spacing_limit`](tests/list_tests.rs#L146) | Yes                   | Yes           | No                  |

### [tests/table\_tests.rs](tests/table_tests.rs) (32)

| Test                                                                                              | Faithful round trip   | Keep Markdown | Existing round trip |
| ------------------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`test_simple_table`](tests/table_tests.rs#L27)                                                   | Configured + expected | Yes           | No                  |
| [`table_rows_do_not_drop_cells_beyond_the_header_width`](tests/table_tests.rs#L59)                | No                    | Yes           | No                  |
| [`malformed_table_foster_parenting_does_not_lose_visible_text`](tests/table_tests.rs#L88)         | No                    | Yes           | No                  |
| [`test_table_with_thead_tbody`](tests/table_tests.rs#L107)                                        | Configured            | Yes           | No                  |
| [`test_table_with_thead_td_headers`](tests/table_tests.rs#L146)                                   | No                    | Yes           | No                  |
| [`test_table_with_caption`](tests/table_tests.rs#L182)                                            | Expected              | Yes           | No                  |
| [`test_table_caption_is_commonmark_only_in_pure_mode`](tests/table_tests.rs#L213)                 | Yes                   | Yes           | No                  |
| [`a_table_with_no_header_row_is_written_as_html`](tests/table_tests.rs#L232)                      | Yes                   | Yes           | No                  |
| [`pure_mode_writes_an_empty_header_row_for_a_headerless_table`](tests/table_tests.rs#L249)        | Yes                   | Yes           | No                  |
| [`extra_header_rows_are_written_as_html`](tests/table_tests.rs#L274)                              | Yes                   | Yes           | No                  |
| [`a_header_cell_in_a_body_row_is_written_as_html`](tests/table_tests.rs#L288)                     | Yes                   | Yes           | No                  |
| [`table_content_a_markdown_row_cannot_hold_is_written_as_html`](tests/table_tests.rs#L301)        | Yes                   | Yes           | No                  |
| [`a_thead_of_data_cells_is_written_as_html`](tests/table_tests.rs#L323)                           | Yes                   | Yes           | No                  |
| [`a_second_tbody_is_written_as_html`](tests/table_tests.rs#L336)                                  | Yes                   | Yes           | No                  |
| [`a_second_thead_is_written_as_html`](tests/table_tests.rs#L351)                                  | Yes                   | Yes           | No                  |
| [`a_thead_after_a_body_row_is_written_as_html`](tests/table_tests.rs#L368)                        | Yes                   | Yes           | No                  |
| [`a_thead_after_an_empty_tbody_is_written_as_html`](tests/table_tests.rs#L388)                    | Yes                   | Yes           | No                  |
| [`a_header_row_after_a_body_row_is_written_as_html`](tests/table_tests.rs#L404)                   | Yes                   | Yes           | No                  |
| [`a_row_wider_or_narrower_than_the_header_is_written_as_html`](tests/table_tests.rs#L415)         | Yes                   | Yes           | No                  |
| [`a_comment_in_a_table_is_written_as_html`](tests/table_tests.rs#L436)                            | Yes                   | Yes           | No                  |
| [`a_pipe_in_an_open_tag_is_encoded`](tests/table_tests.rs#L469)                                   | Yes                   | Yes           | No                  |
| [`a_comment_holding_a_pipe_is_written_as_html`](tests/table_tests.rs#L507)                        | Yes                   | Yes           | No                  |
| [`an_empty_column_is_one_wide`](tests/table_tests.rs#L548)                                        | Configured + expected | Yes           | No                  |
| [`an_empty_table_is_written_as_html`](tests/table_tests.rs#L571)                                  | Yes                   | Yes           | No                  |
| [`test_table_with_inlines`](tests/table_tests.rs#L579)                                            | Configured + expected | Yes           | No                  |
| [`test_table_block_cells`](tests/table_tests.rs#L643)                                             | Configured + expected | Yes           | No                  |
| [`test_hacker_news_layout_table_does_not_turn_into_giant_padded_rows`](tests/table_tests.rs#L675) | No                    | Yes           | No                  |
| [`test_headerless_table`](tests/table_tests.rs#L697)                                              | No                    | Yes           | No                  |
| [`test_headered_with_inner_headerless`](tests/table_tests.rs#L718)                                | No                    | Yes           | No                  |
| [`test_headerless_with_inner_headered`](tests/table_tests.rs#L757)                                | No                    | Yes           | No                  |
| [`a_row_directly_under_a_table_is_read_as_a_body_row`](tests/table_tests.rs#L801)                 | No                    | Yes           | No                  |
| [`test_headered_with_inner_headered`](tests/table_tests.rs#L863)                                  | No                    | Yes           | No                  |

### [tests/translation\_approach\_tests.rs](tests/translation_approach_tests.rs) (16)

| Test                                                                                     | Faithful round trip   | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`translating_html_nodes`](tests/translation_approach_tests.rs#L16)                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L26)                       | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L88)                  | Expected              | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L134)            | Configured + expected | Yes           | No                  |
| [`empty_code_span`](tests/translation_approach_tests.rs#L169)                            | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L202)               | Expected              | Yes           | No                  |
| [`brs_alone_in_a_raw_html_inline`](tests/translation_approach_tests.rs#L263)             | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L286)                                      | Yes                   | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L317)                       | Expected              | Yes           | No                  |
| [`setext_headings`](tests/translation_approach_tests.rs#L340)                            | Configured            | Yes           | No                  |
| [`an_empty_setext_heading_falls_back_to_atx`](tests/translation_approach_tests.rs#L361)  | Configured + expected | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L376)                               | Expected              | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L405)                                 | Yes                   | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L440)                                | Expected              | Yes           | No                  |
| [`tables`](tests/translation_approach_tests.rs#L468)                                     | Configured            | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L493) | Yes                   | Yes           | No                  |

### [tests/turndown\_cases\_tests.rs](tests/turndown_cases_tests.rs) (1)

| Test                                             | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------ | ------------------- | ------------- | ------------------- |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) | No                  | Yes           | No                  |

### [tests/turndown\_online\_demo\_tests.rs](tests/turndown_online_demo_tests.rs) (1)

| Test                                                  | Faithful round trip | Keep Markdown | Existing round trip |
| ----------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`run_cases`](tests/turndown_online_demo_tests.rs#L9) | No                  | Yes           | No                  |

Unit tests and documentation examples
-------------------------------------

The 37 unit tests below exercise parsers, escaping helpers, Unicode data, and
other internal behavior. They do not run the complete Faithful conversion
pipeline, so retain their existing assertions. The last unit test is ignored by
the normal test run because it generates a table.

### [src/html\_block.rs](src/html_block.rs) (9)

| Test                                                                  | Faithful round trip | Keep Markdown | Existing round trip |
| --------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`recognizes_type_1_through_6_prefixes`](src/html_block.rs#L169)      | No                  | N/A           | N/A                 |
| [`applies_the_type_1_and_6_name_terminators`](src/html_block.rs#L186) | No                  | N/A           | N/A                 |
| [`ends_a_type_1_or_6_name_at_the_line_end`](src/html_block.rs#L196)   | No                  | N/A           | N/A                 |
| [`recognizes_a_lone_type_7_tag`](src/html_block.rs#L204)              | No                  | N/A           | N/A                 |
| [`recognizes_a_lone_closing_type_1_tag`](src/html_block.rs#L218)      | No                  | N/A           | N/A                 |
| [`rejects_a_type_7_tag_which_is_not_alone`](src/html_block.rs#L224)   | No                  | N/A           | N/A                 |
| [`rejects_an_incomplete_or_invalid_tag`](src/html_block.rs#L233)      | No                  | N/A           | N/A                 |
| [`allows_three_spaces_of_indentation`](src/html_block.rs#L247)        | No                  | N/A           | N/A                 |
| [`tests_only_the_first_line`](src/html_block.rs#L257)                 | No                  | N/A           | N/A                 |

### [src/element\_handler/element\_util.rs](src/element_handler/element_util.rs) (3)

| Test                                                                                            | Faithful round trip | Keep Markdown | Existing round trip |
| ----------------------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`encodes_the_line_endings_and_pipes_of_an_open_tag`](src/element_handler/element_util.rs#L323) | No                  | Yes           | N/A                 |
| [`escapes_blank_lines_for_each_line_ending_style`](src/element_handler/element_util.rs#L335)    | No                  | Yes           | N/A                 |
| [`escapes_only_the_lines_which_are_blank`](src/element_handler/element_util.rs#L345)            | No                  | Yes           | N/A                 |

### [src/element\_handler/list.rs](src/element_handler/list.rs) (1)

| Test                                                    | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`test_count_digits`](src/element_handler/list.rs#L167) | No                  | N/A           | N/A                 |

### [src/util/escape.rs](src/util/escape.rs) (16)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`escapes_basic_tags`](src/util/escape.rs#L280)                                    | No                  | Yes           | N/A                 |
| [`escapes_misc_sequences`](src/util/escape.rs#L290)                                | No                  | Yes           | N/A                 |
| [`leaves_non_html_sequences`](src/util/escape.rs#L301)                             | No                  | Yes           | N/A                 |
| [`escapes_an_email_autolink`](src/util/escape.rs#L311)                             | No                  | Yes           | N/A                 |
| [`leaves_a_near_miss_email_autolink`](src/util/escape.rs#L319)                     | No                  | Yes           | N/A                 |
| [`escapes_a_sequence_cut_short`](src/util/escape.rs#L346)                          | No                  | Yes           | N/A                 |
| [`escapes_a_literal_cdata_section`](src/util/escape.rs#L365)                       | No                  | Yes           | N/A                 |
| [`leaves_an_escaped_cdata_section`](src/util/escape.rs#L379)                       | No                  | Yes           | N/A                 |
| [`escapes_a_link_destination_backslash`](src/util/escape.rs#L390)                  | No                  | Yes           | N/A                 |
| [`brackets_a_link_destination_the_bare_form_cannot_hold`](src/util/escape.rs#L399) | No                  | Yes           | N/A                 |
| [`escapes_a_link_destination_ampersand`](src/util/escape.rs#L410)                  | No                  | Yes           | N/A                 |
| [`escapes_a_title_backslash`](src/util/escape.rs#L417)                             | No                  | Yes           | N/A                 |
| [`escapes_a_title_ampersand_and_line_endings`](src/util/escape.rs#L423)            | No                  | Yes           | N/A                 |
| [`escapes_an_image_description_as_text`](src/util/escape.rs#L429)                  | No                  | Yes           | N/A                 |
| [`test_index_of_markdown_ordered_item_delimiter`](src/util/escape.rs#L437)         | No                  | N/A           | N/A                 |
| [`a_unicode_numeric_opens_no_ordered_item`](src/util/escape.rs#L457)               | No                  | N/A           | N/A                 |

### [src/util/unicode.rs](src/util/unicode.rs) (8)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`table_is_sorted_and_merged`](src/util/unicode.rs#L33)                            | No                  | N/A           | N/A                 |
| [`agrees_with_a_linear_scan`](src/util/unicode.rs#L49)                             | No                  | N/A           | N/A                 |
| [`classifies_punctuation_and_symbols`](src/util/unicode.rs#L66)                    | No                  | N/A           | N/A                 |
| [`rejects_everything_else`](src/util/unicode.rs#L84)                               | No                  | N/A           | N/A                 |
| [`table_matches_unicode_general_categories`](src/util/unicode.rs#L133)             | No                  | N/A           | N/A                 |
| [`ascii_punctuation_matches_unicode_general_categories`](src/util/unicode.rs#L155) | No                  | N/A           | N/A                 |
| [`header_names_the_unicode_version_of_the_data`](src/util/unicode.rs#L168)         | No                  | N/A           | N/A                 |
| [`print_punctuation_or_symbol_table`](src/util/unicode.rs#L182)                    | No                  | N/A           | N/A                 |

### Documentation examples

| Example                                          | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------ | ------------------- | ------------- | ------------------- |
| `src/lib.rs: convert`                            | No                  | Yes           | N/A                 |
| `src/lib.rs: HtmlToMarkdown`                     | No                  | Yes           | N/A                 |
| `src/lib.rs: HtmlToMarkdownBuilder::add_handler` | No                  | N/A           | N/A                 |
| `src/element_handler/mod.rs: EventTypes`         | No                  | N/A           | N/A                 |
