# LLM Policy

Usage of LLMs in the development of Voxy is **allowed with caveats**. By using
an LLM you are expected to do so responsibly, and agree to adhere to the following 
statements.

---

> [!Warning]
Failure to adhere to these expectations on a consistent basis may restrict
your ability to contribute to the Voxy project as a whole. This doesn't mean
you are expected to be perfect every single time. So long as it is determined
that you have put in the effort to meet these standards, you will not be
subject to this warning.

> [!Important]
Every PR must be tagged with either `ai-assisted` or `no-ai`. Note that
neither tag carries more weight than the other, and only exists as a mechanism
to assist both you and our reviewers. Just because a PR is tagged as `no-ai`
doesn't mean it won't undergo scrutiny; if it is suspected that a PR has lied
about its usage status, the [review process](#the-review-process) will be
escalated. If you believe our reviewers acted in bad-faith, please file an
[issue](https://github.com/BoringOrng/voxy/issues).
>
> It's understandable that people make mistakes. The first occurrence will be
assumed to be a mistake. For first-time contributors, the second occurrence
will be considered a mistake as well. "First-time contributor" means this is
the contributor's first ever PR to the project; if that PR is found to be in
violation of this policy, the contributor receives the two-warning leniency.
Should you make a mistake, you must be adequately warned, that is, the parts
of your PR that are believed to be LLM-influenced must be pointed out by the
reviewer, and you must receive a link to this policy. If the reviewer fails
to adequately warn you, the warning will be waived. Additionally, failing to
properly disclose LLM usage will be waived if the PR description contains 
how an LLM was used.
>
> Should all warnings be exhausted, and you are adequately found in violation
of this policy, your ability to contribute to the Voxy project as a whole will
be restricted. To be found adequately in violation of this policy, all of the
following must be true:
> - You have been adequately warned, as defined above, twice as a first-time
    contributor, or once otherwise.
> - The PR determining your restriction has gone through standard review
    procedure, and the reviewer has flagged suspected LLM usage.
> - The reviewer has forwarded the PR to @BoringOrng, and @BoringOrng has,
    to the best of their ability, determined that the current PR and all prior
    warnings carry sufficient evidence of improper disclosure of LLM usage.
>
> For those of you that did have AI assistance, it's encouraged to state how it
  was used, as this achieves a few things:
> - We can understand how AI is used, and thus refine our policy;
> - Our reviewers can be more fair and understand better where you're coming from;
> - PR reviews will be faster, as the reviewer will know where to pay more attention
    to.

---

## LLMs Don't Produce Finished Code

LLM-authored code shouldn't reach production in a raw state. For the sake of
protecting all individuals whom contribute to the codebase, and to the codebase
itself, this is an absolute must. LLM-authored code (as of Aug 2, 2026) must
[". . . reflect sufficient human contribution to warrant copyright protection."
](#copyright-and-artificial-intelligence-part-2-copyrightability-report)
This is especially important to Voxy, as it is dual-licensed under
[MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE). Code solely produced
by an LLM cannot be protected under these licenses as it doesn't meet the minimum
standard of sufficient human contribution, thus stripping users of the protections
these licenses provide.

## Code Influence or Suggestion

Any code influenced or suggested by an LLM must be actively refactored,
restructured, and rewritten to match the Voxy-specific architectural standards.
Once again, this requirement is in place to inject the necessary human creative
input to be legally protect-able under our open-source licenses.

> [!Important]
Refactors, restructuring, and rewriting must be done by a human, and to the
fullest extent of sufficient human contribution. [Prompting an LLM isn't
considered sufficient human contribution.
](#copyright-and-artificial-intelligence-part-2-copyrightability-report)

## Code Competency

LLMs are text-prediction engines, and they don't understand software architecture
or the specific nuances of this codebase. LLMs must never be used as a substitute
for understanding the project. If you don't understand the codebase, libraries,
or language well enough to write the feature yourself, you shouldn't use an LLM
to write it for you. You must remember, you are the author of the code, and others
will rely on said code, if something breaks you are expected to be able to fix it.

> [!Important]
In submitting LLM-influenced code, you may be subject to a few code-competency
questions. We do this not to undermine or doubt your abilities, but as assurance
that you have met this standard.

## Pull Request Requirements

Submitting LLM-influenced code may subject your PR to more vigorous requirements,
as the reviewer deems necessary. If you suspect a reviewer to be injecting their
own beliefs, biases or are treating you unfairly for using LLM-influenced code,
please open an [issue](https://github.com/BoringOrng/voxy/issues) as this will
not be tolerated.

> [!Important]
For the following segments, English is the preferred language. If you cannot
speak English, please provide the translated text along with the original text.

### Creation of an LLM-Influenced PR

Creating a PR with LLM-influenced code will have the following restrictions.
- The PR must explicitly state that it used LLM-influenced code.
- The PR must be created as draft.
- The review process will be stricter and more involved.

---

> [!Important]
Some of these restrictions may not apply for simple fixes at the reviewer's
discretion.

### The Review Process

The review process for LLM-influenced code, is stricter and more involved for
both the author, and the reviewer.

#### The Review Process for the Reviewer

Your job as the reviewer is **not** to judge the PR's author for LLM usage. As
the reviewer, you must provide assurance that the code meets the Voxy standards,
and is sufficiently human-created. You have the right to deny a PR, and warn the
author, should you believe they haven't adhered to the LLM policy as is authored
here. Injection of your own beliefs, or biases will not be tolerated on any metric,
and the author of the PR has the right to report you for it. Should you be reported,
and the report verified, you may be barred from reviewing in the future, and
potentially contributing to the Voxy project as a whole. Likewise, if it is
determined that you are not upholding your responsibilities in reviewing the code,
the same punishments apply. Neither leniency nor pragmatism will be tolerated.

> [!Important]
If you do not believe you can adhere to these standards, you cannot be forced
to review LLM-influenced code.

#### The Review Process for the Author

Assuming you have created your PR as draft and have to the best of your ability
ensured that your code is a reflection of your human creativity, your first step is
to wait. It may take longer for a reviewer to get begin reviewing your code as the
[review-process is fundamentally different.](#the-review-process-for-the-reviewer)

You may have to answer competency questions. You are expected to answer these
questions in your own words. If a reviewer asks you to explain a design-decision,
a lifetime, an `unsafe` block, or why a refactor took the shape it did, the answer
should come from your own understanding. You cannot use an LLM to refine your
answer, or answer for you. Clarifying questions may be asked, and you are expected
to answer them to the same standard. Failure to answer a competency question
doesn't make you a failure, it simply means the PR isn't ready yet. You reserve
the right to ask the reviewer what you need to better understand before
resubmitting.

Disagreement isn't bias. The right to report a reviewer for injecting bias exists
for cases of unfair treatment. When a reviewer gives you push-back, or questions
the code quality, it isn't a sign that you are being discriminated against.
Oftentimes when a reviewer's concern is substantive, engaging with it will be an
overall benefit to all parties involved. That doesn't mean doing everything exactly
as the reviewer wants you to, the reviewer is human too. If you believe your
solution to be better than what the reviewer suggests, discuss it with them, this
will likely make the review process faster, as it will express a level of
competency.

## Sources

### [Copyright and Artificial Intelligence Part 2 Copyrightability Report](https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf)
